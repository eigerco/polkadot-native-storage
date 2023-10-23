// Imports created by construct_runtime macros are unresolved by rust analyzer
use frame_support::{assert_noop, assert_ok, dispatch::DispatchResultWithPostInfo};
use pallet_pns_common::{address::Address, AccountIdConversion, MinerId};

use crate as pallet_miner;
use crate::{mock::*, Error, MinerControllers};

const WORKER: Address = Address::new_id(1);
const PEERID_BYTE: u8 = 9;
const FIRST_MINER_ADDR: u64 = 1590839634285;

// Utility functions
//
// TODO: add genesis config in pallet and build test_ext with it
fn create_miner_for(
    owner: <Test as frame_system::Config>::AccountId,
) -> DispatchResultWithPostInfo {
    MinerModule::create(RuntimeOrigin::signed(1), owner, WORKER, vec![PEERID_BYTE])
}

#[test]
fn create_miner_first_miner_addr_is_correct() {
    new_test_ext().execute_with(|| {
        let new_miner_addr: <Test as frame_system::Config>::AccountId = MinerId(1).into_account();
        assert_eq!(new_miner_addr, FIRST_MINER_ADDR);
    })
}

#[test]
fn create_miner() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 0;
        let worker = WORKER;
        let peer_id = vec![1, 32];
        let expected_miner_index = 1;

        // this needs to be set in order to read back the System::events later on, Events are not
        // populated on genesis - unsure why
        // https://github.com/paritytech/substrate/blob/master/frame/system/src/lib.rs#L1122
        System::set_block_number(1);

        assert_ok!(MinerModule::create(
            RuntimeOrigin::signed(1),
            owner,
            worker,
            peer_id.clone()
        ));

        let miner_index = MinerModule::miner_index();
        let new_miner_addr: <Test as frame_system::Config>::AccountId =
            MinerId(miner_index.unwrap()).into_account();
        let new_miner_info = MinerModule::miners(new_miner_addr).unwrap();

        assert_eq!(MinerModule::miner_index(), Some(expected_miner_index));
        assert_eq!(new_miner_info.owner, owner);
        assert_eq!(new_miner_info.worker, worker);
        assert_eq!(new_miner_info.peer_id, peer_id);
        assert_eq!(new_miner_info.controllers.len(), 0);
        assert_eq!(System::event_count(), 1);

        //System::assert_last_event(RuntimeEvent::MinerModule(Event::MinerCreated{miner_account_id: new_miner_addr}));
    });
}

#[test]
fn change_worker_address_works_with_valid_signer_and_new_worker() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events and calculation for effective_at
        let block = 1;
        System::set_block_number(block);

        let new_worker: Address = Address::new_id(111);
        let new_controllers = MinerControllers::Override(vec![1, 2, 3]);
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            new_controllers.clone()
        ));

        let miner_key_change = MinerModule::miners(FIRST_MINER_ADDR)
            .unwrap()
            .pending_worker
            .unwrap();

        assert_eq!(miner_key_change.new_worker, new_worker);
        assert_eq!(
            miner_key_change.effective_at,
            block + <Test as pallet_miner::Config>::BlockDelay::get()
        )
    });
}

#[test]
fn change_worker_address_clears_pending_worker_with_valid_signer_and_old_worker() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial pending_worker request
        let new_worker: Address = Address::new_id(111);
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::NoChange
        ));

        // clears existing pending_worker request with existing worker
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            WORKER,
            MinerControllers::NoChange
        ));

        assert!(MinerModule::miners(FIRST_MINER_ADDR)
            .unwrap()
            .pending_worker
            .is_none());
    })
}

#[test]
fn change_worker_address_keeps_old_controller_without_override() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial pending_worker request
        let new_worker: Address = Address::new_id(111);
        let new_controllers = vec![1, 2, 3];
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::Override(new_controllers.clone())
        ));

        // clears existing pending_worker request with existing worker
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            WORKER,
            MinerControllers::NoChange
        ));

        assert_eq!(
            MinerModule::miners(FIRST_MINER_ADDR).unwrap().controllers,
            new_controllers
        );
    })
}

#[test]
fn change_worker_address_rejects_invalid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events
        System::set_block_number(1);

        // set initial pending_worker request
        let invalid_signer: u64 = 456;
        assert_noop!(
            MinerModule::change_worker_address(
                RuntimeOrigin::signed(invalid_signer),
                FIRST_MINER_ADDR,
                WORKER,
                MinerControllers::NoChange
            ),
            Error::<Test>::InvalidSigner
        );
    })
}

#[test]
fn confirm_update_worker_accepts_effective_request_with_valid_signature() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events
        System::set_block_number(1);

        // set initial pending_worker request
        let new_worker: Address = Address::new_id(111);
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::NoChange
        ));

        System::set_block_number(10);

        assert_ok!(MinerModule::confirm_update_worker_key(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
        ));

        let new_miner_info = MinerModule::miners(FIRST_MINER_ADDR).unwrap();

        assert_eq!(new_miner_info.worker, new_worker);
        assert!(new_miner_info.pending_worker.is_none());
    })
}

#[test]
fn confirm_update_worker_key_rejects_trigger_before_effective_at() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events
        System::set_block_number(1);

        // set initial pending_worker request
        let new_worker: Address = Address::new_id(111);
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::NoChange
        ));

        System::set_block_number(<Test as pallet_miner::Config>::BlockDelay::get());

        assert_noop!(
            MinerModule::confirm_update_worker_key(RuntimeOrigin::signed(owner), FIRST_MINER_ADDR,),
            Error::<Test>::IneffectiveRequest
        );
    })
}

#[test]
fn change_worker_address_rejects_trigger_without_request() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        assert_noop!(
            MinerModule::confirm_update_worker_key(RuntimeOrigin::signed(owner), FIRST_MINER_ADDR,),
            Error::<Test>::NoRequest
        );
    })
}

#[test]
fn change_owner_address_creates_proposal_with_valid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        assert_ok!(create_miner_for(owner));

        System::set_block_number(1);
        assert_ok!(MinerModule::change_owner_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_eq!(
            MinerModule::miners(FIRST_MINER_ADDR).unwrap().pending_owner,
            Some(new_owner)
        );
    })
}

#[test]
fn change_owner_address_rejects_proposal_with_owner_account() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        assert_noop!(
            MinerModule::change_owner_address(
                RuntimeOrigin::signed(owner),
                FIRST_MINER_ADDR,
                owner,
            ),
            Error::<Test>::IneffectiveRequest
        );
    })
}

#[test]
fn change_owner_address_rejects_invalid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        let random_account: u64 = 789;
        assert_ok!(create_miner_for(owner));

        assert_noop!(
            MinerModule::change_owner_address(
                RuntimeOrigin::signed(random_account),
                FIRST_MINER_ADDR,
                new_owner
            ),
            Error::<Test>::InvalidSigner
        );
    })
}

#[test]
fn change_owner_address_confirms_new_owner_with_valid_signer_and_proposal() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        assert_ok!(create_miner_for(owner));

        assert_ok!(MinerModule::change_owner_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        System::set_block_number(1);
        assert_ok!(MinerModule::change_owner_address(
            RuntimeOrigin::signed(new_owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_eq!(
            MinerModule::miners(FIRST_MINER_ADDR).unwrap().owner,
            new_owner
        );
        assert_eq!(
            MinerModule::miners(FIRST_MINER_ADDR).unwrap().pending_owner,
            None
        )
    })
}

#[test]
fn change_owner_address_revokes_existing_proposal_with_valid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        assert_ok!(create_miner_for(owner));

        assert_ok!(MinerModule::change_owner_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_ok!(MinerModule::change_owner_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_eq!(
            MinerModule::miners(FIRST_MINER_ADDR).unwrap().pending_owner,
            Some(new_owner)
        );

        System::set_block_number(1);

        assert_ok!(MinerModule::change_owner_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            owner
        ));

        assert_eq!(MinerModule::miners(FIRST_MINER_ADDR).unwrap().owner, owner);
        assert_eq!(
            MinerModule::miners(FIRST_MINER_ADDR).unwrap().pending_owner,
            None
        )
    })
}

#[test]
fn change_peer_id_works_with_valid_owner() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_peer_id = vec![88];
        assert_ok!(create_miner_for(owner));

        System::set_block_number(1);

        assert_ok!(MinerModule::change_peer_id(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            new_peer_id.clone()
        ));

        let peer_id = MinerModule::miners(FIRST_MINER_ADDR).unwrap().peer_id;

        assert_eq!(peer_id, new_peer_id)
    });
}

#[test]
fn change_peer_id_works_with_valid_controller() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_peer_id = vec![88];
        let new_controllers = vec![1, 2, 3];
        assert_ok!(create_miner_for(owner));

        // Add controllers to new miner
        assert_ok!(MinerModule::change_worker_address(
            RuntimeOrigin::signed(owner),
            FIRST_MINER_ADDR,
            WORKER,
            MinerControllers::Override(new_controllers.clone())
        ));

        System::set_block_number(1);

        assert_ok!(MinerModule::change_peer_id(
            RuntimeOrigin::signed(new_controllers[0]),
            FIRST_MINER_ADDR,
            new_peer_id.clone()
        ));

        let peer_id = MinerModule::miners(FIRST_MINER_ADDR).unwrap().peer_id;

        assert_eq!(peer_id, new_peer_id)
    });
}

#[test]
fn change_peer_id_rejects_invalid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        // This signer is not the owner, worker or a controller of the miner
        let invalid_signer: u64 = 234;
        let new_peer_id = vec![88];

        assert_ok!(create_miner_for(owner));
        assert_noop!(
            MinerModule::change_peer_id(
                RuntimeOrigin::signed(invalid_signer),
                FIRST_MINER_ADDR,
                new_peer_id
            ),
            Error::<Test>::InvalidSigner
        );
    });
}
