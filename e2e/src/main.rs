#![warn(unused_extern_crates)]

fn main() {}

#[cfg(test)]
pub mod tests {
	use std::num::NonZeroU16;

	use avail_core::{AppExtrinsic, AppId, Keccak256};
	use avail_subxt::api;
	use avail_subxt::{
		api::runtime_types::avail_core::header::extension::HeaderExtension,
		avail::{Cells, GDataProof, GRawScalar, GRow, Rows},
		rpc::KateRpcClient as _,
		submit::submit_data,
		tx,
		utils::H256,
		AvailClient, Cell,
	};

	use anyhow::{anyhow, Result};
	use avail_subxt::api::runtime_types::avail_core::{BlockLengthColumns, BlockLengthRows};
	use binary_merkle_tree::merkle_proof;
	use kate::gridgen::{AsBytes as _, EvaluationGrid};
	use kate_recovery::matrix::Dimensions;
	use kate_recovery::proof::verify;
	use sp_core::{keccak_256, U256};
	use subxt_signer::sr25519::dev;

	pub const MIN_WIDTH: usize = 4;
	pub const DATA: &[u8] = b"ExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleDataExampleData";
	pub const TWO_FULL_ROWS: &str = include_str!("./res/two_full_rows.txt");
	pub const ONE_FULL_ROW: &str = include_str!("./res/one_full_row.txt");

	async fn establish_a_connection() -> Result<AvailClient> {
		let ws = String::from("ws://127.0.0.1:9944");
		let client = AvailClient::new(ws).await?;
		Ok(client)
	}

	/* 	#[async_std::test]
	   pub async fn rpc_query_rows_test_1() -> Result<()> {
		   let client = establish_a_connection().await?;
		   let alice = dev::alice();

		   let data = ONE_FULL_ROW.to_ascii_lowercase();
		   let data = data.as_bytes();

		   println!("-----");
		   println!("Test 1: Submitting one full row of data.");
		   println!("-----");
		   let block_hash = tx::in_finalized(submit_data(&client, &alice, data, AppId(1)).await?)
			   .await?
			   .block_hash();

		   print!("If only one row is submited then there should not be a second row.");
		   let rows = client
			   .rpc_methods()
			   .query_rows(Rows::truncate_from(vec![1]), block_hash)
			   .await;
		   assert!(rows.is_err(), "Got two rows. This is bad");
		   println!(" Passed ✅");

		   print!("Fetching the first row.");
		   let rows = client
			   .rpc_methods()
			   .query_rows(Rows::truncate_from(vec![0]), block_hash)
			   .await;
		   assert!(rows.is_ok(), "Got no rows. This is bad");
		   println!(" Passed ✅");
		   let rows = rows.unwrap();

		   print!("There should be 256 U256 elements in the first row.");
		   assert_eq!(rows[0].len(), 256, "Didn't get 256 U256 elements");
		   println!(" Passed ✅");

		   print!("First element in the row should be U256: `2242984242519499925972102513023760885346123943699308488855333492107061879552.`");
		   assert_eq!(
			   rows[0][0].to_string(),
			   String::from(
				   "2242984242519499925972102513023760885346123943699308488855333492107061879552"
			   ),
			   "First element in the row didn't match"
		   );
		   println!(" Passed ✅");

		   println!("Skipping second element...");
		   println!("Skipping third element...");
		   println!("Skipping fourth element...");

		   print!("The rest of them in the row should be U256: `22250244598543112061784228491865519548275408818103480933072241632893111185664.`");
		   for v in &rows[0][4..] {
			   assert_eq!(
				   v.to_string(),
				   String::from(
					   "22250244598543112061784228491865519548275408818103480933072241632893111185664"
				   ),
				   "One element in the row didn't match"
			   );
		   }
		   println!(" Passed ✅");

		   Ok(())
	   }
	*/
	#[async_std::test]
	pub async fn rpc_query_rows_test_2() -> Result<()> {
		let client = establish_a_connection().await?;
		let alice = dev::alice();

		println!("-----");
		println!("Test 2: Submitting two full rows of data.");
		println!("-----");

		let data = TWO_FULL_ROWS.to_ascii_lowercase();
		let data = data.as_bytes();
		let block_hash = tx::in_finalized(submit_data(&client, &alice, data, AppId(1)).await?)
			.await?
			.block_hash();

		print!("If only two rows is submited then there should not be a third row.");
		let rows = client
			.rpc_methods()
			.query_rows(Rows::truncate_from(vec![2]), block_hash)
			.await;
		assert!(rows.is_err(), "Got three rows. This is bad");
		println!(" Passed ✅");

		print!("Fetching the first two rows.");
		let rows = client
			.rpc_methods()
			.query_rows(Rows::truncate_from(vec![0, 1]), block_hash)
			.await;
		assert!(rows.is_ok(), "Got no rows. This is bad");
		println!(" Passed ✅");
		let rows = rows.unwrap();

		print!("The last 252 elements in the first row should be the same as the last 256 elements in the second row. U256: `22250244598543112061784228491865519548275408818103480933072241632893111185664.`");
		for v in &rows[1][4..] {
			assert_eq!(
				v.to_string(),
				String::from(
					"22250244598543112061784228491865519548275408818103480933072241632893111185664"
				),
				"One element in the first row didn't match"
			);
		}
		for v in &rows[1] {
			assert_eq!(
				v.to_string(),
				String::from(
					"22250244598543112061784228491865519548275408818103480933072241632893111185664"
				),
				"One element in the second row didn't match"
			);
		}
		println!(" Passed ✅");

		Ok(())
	}
	/*
	#[async_std::test]
	pub async fn rpc_query_app_data_test() -> Result<()> {
		let client = establish_a_connection().await?;
		let alice = dev::alice();
		let app_id = AppId(1);

		let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, app_id).await?)
			.await?
			.block_hash();
		let grid = eval_grid_from_block(&client, block_hash).await?;

		// RPC call
		let actual_rows = client
			.rpc_methods()
			.query_app_data(app_id, block_hash)
			.await?;

		let row = grid.row(0).unwrap();
		let expected_rows = row
			.iter()
			.flat_map(|r| r.to_bytes().map(GRawScalar::from))
			.collect::<Vec<_>>();
		assert_eq!(actual_rows, vec![Some(expected_rows)]);
		Ok(())
	}

	#[async_std::test]
	pub async fn rpc_query_proof_test() -> Result<()> {
		let client = establish_a_connection().await?;
		let alice = dev::alice();
		let app_id = AppId(1);

		let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, app_id).await?)
			.await?
			.block_hash();
		let grid = eval_grid_from_block(&client, block_hash).await?;

		// Grid Creation
		let extended_grid = grid.extend_columns(NonZeroU16::new(2).unwrap()).unwrap();
		let poly_grid = extended_grid.make_polynomial_grid().unwrap();

		assert_eq!(grid.dims(), Dimensions::new(1, 4).unwrap());
		assert_eq!(extended_grid.dims(), Dimensions::new(2, 4).unwrap());
		assert_eq!(grid.row(0), extended_grid.row(0));
		assert_eq!(grid.row(0), extended_grid.row(1));

		let cells = (0..grid.dims().cols().into())
			.map(|col| Cell::new(0, col.into()))
			.collect::<Vec<_>>();
		let multiproof_srs = kate::couscous::multiproof_params();
		let expected_proof: Vec<Vec<u8>> = cells
			.iter()
			.map(|cell| {
				let row = usize::try_from(cell.row).unwrap();
				let col = usize::try_from(cell.col).unwrap();
				let data = extended_grid.get::<usize, usize>(row, col).unwrap();
				let kate_cell = kate::com::Cell::new(
					avail_core::BlockLengthRows(cell.row),
					avail_core::BlockLengthColumns(cell.col),
				);
				let proof = poly_grid.proof(&multiproof_srs, &kate_cell).unwrap();
				let data = data.to_bytes().expect("Ser cannot fail").to_vec();
				let proof = proof.to_bytes().expect("Ser cannot fail").to_vec();

				[proof, data].into_iter().flatten().collect::<Vec<u8>>()
			})
			.collect();
		let expected_proof: Vec<u8> = expected_proof.into_iter().flatten().collect();

		// RPC call
		let actual_proof: Vec<GDataProof> = client
			.rpc_methods()
			.query_proof(Cells::try_from(cells).unwrap(), block_hash)
			.await?;
		let actual_proof: Vec<u8> = actual_proof
			.iter()
			.map(|(raw_scalar, g_proof)| {
				let mut scalar_bytes = [0u8; 32];
				raw_scalar.to_big_endian(&mut scalar_bytes);
				let proof_bytes: Vec<u8> = Vec::from(*g_proof);

				[proof_bytes, scalar_bytes.to_vec()]
					.into_iter()
					.flatten()
					.collect::<Vec<u8>>()
			})
			.flatten()
			.collect();

		assert_eq!(actual_proof, expected_proof);
		Ok(())
	}

	#[async_std::test]
	pub async fn rpc_query_proof_test_2() -> Result<()> {
		let client = establish_a_connection().await.unwrap();
		let alice = dev::alice();

		let mut example_data = [0u8; 12_500];
		example_data[..7].copy_from_slice(b"example");
		assert_eq!(example_data.len(), 12_500);

		let block_hash =
			tx::in_finalized(submit_data(&client, &alice, &example_data, AppId(1)).await?)
				.await?
				.block_hash();

		let cell = Cell { row: 0, col: 0 };
		let cells = Cells::try_from(vec![cell.clone()]).unwrap();

		// RPC call
		let actual_proof: Vec<GDataProof> =
			client.rpc_methods().query_proof(cells, block_hash).await?;
		let actual_proof: Vec<u8> = actual_proof
			.iter()
			.map(|(raw_scalar, g_proof)| {
				let mut scalar_bytes = [0u8; 32];
				raw_scalar.to_big_endian(&mut scalar_bytes);
				let proof_bytes: Vec<u8> = Vec::from(*g_proof);

				[proof_bytes, scalar_bytes.to_vec()]
					.into_iter()
					.flatten()
					.collect::<Vec<u8>>()
			})
			.flatten()
			.collect();
		assert_eq!(actual_proof.len(), 80);

		let pp = kate::couscous::public_params();

		let submitted_block = client.blocks().at(block_hash).await?;

		let (commitment, rows, cols) = match &submitted_block.header().extension {
			HeaderExtension::V3(ext) => (
				ext.commitment.commitment.clone(),
				ext.commitment.rows,
				ext.commitment.cols,
			),
		};

		let mut content = [0u8; 80];
		content.copy_from_slice(&actual_proof);
		let commitment: [u8; 48] = commitment[..48].try_into().unwrap();
		let dcell = kate_recovery::data::Cell {
			position: kate_recovery::matrix::Position { row: 0, col: 0 },
			content,
		};
		let dim = Dimensions::new(rows, cols).unwrap();
		let res = verify(&pp, dim, &commitment, &dcell).unwrap();
		assert!(res);

		// Fetch & verify extended cell
		let cell = Cell { row: 1, col: 1 };
		let cells = Cells::try_from(vec![cell.clone()]).unwrap();

		// RPC call
		let actual_proof: Vec<GDataProof> =
			client.rpc_methods().query_proof(cells, block_hash).await?;
		let actual_proof: Vec<u8> = actual_proof
			.iter()
			.map(|(raw_scalar, g_proof)| {
				let mut scalar_bytes = [0u8; 32];
				raw_scalar.to_big_endian(&mut scalar_bytes);
				let proof_bytes: Vec<u8> = Vec::from(*g_proof);

				[proof_bytes, scalar_bytes.to_vec()]
					.into_iter()
					.flatten()
					.collect::<Vec<u8>>()
			})
			.flatten()
			.collect();
		assert_eq!(actual_proof.len(), 80);

		let (commitment, rows, cols) = match &submitted_block.header().extension {
			HeaderExtension::V3(ext) => (
				ext.commitment.commitment.clone(),
				ext.commitment.rows,
				ext.commitment.cols,
			),
		};

		let mut content = [0u8; 80];
		content.copy_from_slice(&actual_proof);
		let commitment: [u8; 48] = commitment[48..96].try_into().unwrap();
		let dcell = kate_recovery::data::Cell {
			position: kate_recovery::matrix::Position { row: 1, col: 1 },
			content,
		};
		let dim = Dimensions::new(rows, cols).unwrap();
		let res = verify(&pp, dim, &commitment, &dcell).unwrap();
		assert!(res);
		Ok(())
	}

	#[async_std::test]
	pub async fn empty_commitments_test() -> Result<()> {
		let client = establish_a_connection().await.unwrap();
		let alice = dev::alice();
		// other than DA tx
		let call = api::tx().system().remark(b"Hi".to_vec());
		let block_hash = tx::send_then_finalized(&client, &call, &alice, AppId(0))
			.await?
			.block_hash();

		// query_rows should fail for block with empty commitments
		let row_indexes = Rows::truncate_from(vec![0]);
		let rows = client
			.rpc_methods()
			.query_rows(row_indexes, block_hash)
			.await;
		assert!(rows.is_err());

		// query_proof should fail for block with empty commitments
		let cell = Cell { row: 0, col: 0 };
		let cells = Cells::try_from(vec![cell.clone()]).unwrap();

		let proof = client.rpc_methods().query_proof(cells, block_hash).await;
		assert!(proof.is_err());
		Ok(())
	}

	#[async_std::test]
	pub async fn rpc_query_block_length_test() -> Result<()> {
		let client = establish_a_connection().await.unwrap();
		let alice = dev::alice();

		println!("Data submitted...");
		let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, AppId(1)).await?)
			.await?
			.block_hash();

		// RPC call
		let length = client.rpc_methods().query_block_length(block_hash).await?;
		assert_eq!(length.cols, BlockLengthColumns(256));
		assert_eq!(length.rows, BlockLengthRows(256));
		assert_eq!(length.chunk_size, 32);
		Ok(())
	}

	#[async_std::test]
	pub async fn rpc_query_data_proof_test() -> Result<()> {
		let client = establish_a_connection().await?;
		let alice = dev::alice();

		// data hash: 729afe29f4e9fee2624d7ed311bcf57d24683fb78938bcb4e2a6a22c4968795e
		println!("Data submitted...");
		let block_hash = tx::in_finalized(submit_data(&client, &alice, DATA, AppId(1)).await?)
			.await?
			.block_hash();

		let expected_proof_root = merkle_proof::<Keccak256, _, _>(vec![keccak_256(DATA)], 0);

		let actual_proof = client.rpc_methods().query_data_proof(1, block_hash).await?;
		// root is calculated keccak256(blob_root, bridge_root)
		let mut root_data = vec![];
		root_data.extend(expected_proof_root.root.as_bytes());
		root_data.extend(H256::zero().as_bytes());
		let expected_data_root = keccak_256(root_data.as_slice());

		assert_eq!(
			actual_proof.data_proof.roots.data_root,
			H256(expected_data_root)
		);
		assert_eq!(actual_proof.data_proof.proof, expected_proof_root.proof);
		assert_eq!(
			actual_proof.data_proof.roots.blob_root,
			expected_proof_root.root
		);
		assert_eq!(actual_proof.data_proof.roots.bridge_root, H256::zero());
		Ok(())
	} */
}
