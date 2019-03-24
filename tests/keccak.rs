extern crate cryptonight;

use cryptonight::byte_string;
use cryptonight::keccak;

#[test]
fn test_keccak1() {
    let input = byte_string::string_to_u8_array("0505fbf6ffcb050b68956935c6c2902af098f48b969d6e3577647e80c556d90ab2415c2996bb1625004000676466b9986865ae42affe0bf4b86a43129156457c76bd1968d087cc8a1bd46606");
    let a = keccak::keccak(&input);
    assert_eq!(byte_string::u8_array_to_string(&a), "96f97e9a022fe202433b3a4ea641b9c753b2d59c57dd781a6710e028dff5a045967fa1a0dbef99f69abb28cab7983d7ace9566f4162da5cc7ba08e1c8711336e1e47cac953ff75b2464999274d2bca5784932ab7947314389b52b5acdd60aac8d3e974b0780e48905ce519a319f9b05d6b75e660aa18b86a44021de5d3ff4a3c679942f0eb013bdc7af67eaebc38d4dd9c9b9a73f73de8c7ef0e8b6fddf8e207e45446461334be8694c22d339cafb1e236bfa92061f19adb6bc3918d06c5458c827818bc7d69c02a");
}

#[test]
fn test_keccak2() {
    let input = byte_string::string_to_u8_array("0505fbf6ffcb050b68956935c6c2902af098f48b969d6e3577647e80c556d90ab2415c2996bb162400c000676466b9986865ae42affe0bf4b86a43129156457c76bd1968d087cc8a1bd46606");
    let a = keccak::keccak(&input);
    assert_eq!(byte_string::u8_array_to_string(&a), "07160933280461683196fb16638c2d1e890bd7a4b44ffa3760a5eda971f5416bce39ef45d1feddfd76beb38ee76965605900659a5951b538855cd84a3ee8c77a677c83c7d57c159a8fdb55af9f94071b8b8a634106780f167ff64faa26d4ab9f9966236d6b2a0480d98ec0b004834c41a7d9ec6ae02ed0b6c61715cb5e1783567382d7b1d1e567dc7e0c37b2970d564bb1e7fd5ea8c11ecbf0d4a3125276e0d47f0569e36098f6cfbbc63444f66fe9a09133d5984e490273f337c199e5e852dd0251c883228f69a7");
}
