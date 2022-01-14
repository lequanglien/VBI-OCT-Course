use std::fmt::format;
use crate::{mock::*, Error, Photo, Proofs};
use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::OriginFor;

use sp_io::logging::log;

#[test]
fn test_create_claim() {
	new_test_ext().execute_with(|| {
		let claim = Photo {
			name : "test.jpg".as_bytes().to_vec(),
			hash: "sha256".as_bytes().to_vec(),
		};


		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim( Origin::signed(1), claim.clone()));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));

		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));
	});
}



#[test]
fn test_revoke_claim() {
	new_test_ext().execute_with(|| {
		let claim = Photo {
			name : "test.jpg".as_bytes().to_vec(),
			hash: "sha256".as_bytes().to_vec(),
		};


		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim( Origin::signed(1), claim.clone()));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));

		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));

		assert_ok!(TemplateModule::revoke_claim( Origin::signed(1), claim.clone()));

		assert_eq!(Proofs::<Test>::contains_key(claim.clone()), false);

	});
}




#[test]
fn test_transfer_claim() {
	new_test_ext().execute_with(|| {
		let claim = Photo {
			name : "test.jpg".as_bytes().to_vec(),
			hash: "sha256".as_bytes().to_vec(),
		};


		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_claim( Origin::signed(1), claim.clone()));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));

		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));

		assert_ok!(TemplateModule::transfer_claim( Origin::signed(1), 2 , claim.clone()));

		assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Pallet::<Test>::block_number()));

	});
}
