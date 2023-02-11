use frost_signer::signing_round::{DkgBegin, MessageTypes, SignatureShareRequest, SigningRound};
use wtfrost::common::PublicNonce;

#[ignore]
fn setup_signer(_total: usize, _threshold: usize) -> SigningRound {
    todo!()
    // let my_id = 1;
    // let mut signer = SigningRound::new(my_id, threshold, total);
    // signer.reset();
    // signer
}

#[ignore]
#[test]
fn dkg_begin() {
    let total = 2;
    let mut signer = setup_signer(total, total - 1);
    assert_eq!(signer.commitments.len(), 0);

    let dkg_begin_msg = MessageTypes::DkgBegin(DkgBegin { dkg_id: 0 });
    let msgs = signer.process(dkg_begin_msg).unwrap();
    assert_eq!(msgs.len(), total);

    // part of the DKG_BEGIN process is to fill the commitments array
    assert_eq!(signer.commitments.len(), signer.total);
}

#[ignore]
#[test]
fn signature_share() {
    let share = SignatureShareRequest {
        dkg_id: 0,
        correlation_id: 0,
        party_id: 0,
        nonces: [(
            0,
            PublicNonce {
                D: Default::default(),
                E: Default::default(),
            },
        )]
        .to_vec(),
        message: vec![],
    };

    let msg_share = MessageTypes::SignShareRequest(share);

    let mut signer = setup_signer(2, 1);
    signer.process(msg_share).unwrap();
}
