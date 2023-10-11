use super::*;
use crate as pallet_light_verifier;
use crate::mock::{Test, *};
use dkg_runtime_primitives::{TypedChainId, H256};

use frame_support::assert_ok;

use pallet_eth2_light_client::tests::{get_test_context, submit_and_check_execution_headers};
use sp_runtime::AccountId32;

use webb_proposals::{self};

pub const GOERLI_CHAIN: TypedChainId = TypedChainId::Evm(5);
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

#[test]
fn test_verify_storage_proof_positive_case() {
	new_test_ext().execute_with(|| {
	// prep light client pallet
	let (headers, updates, _init_input) = get_test_context(None);
	assert_ok!(Eth2Client::submit_beacon_chain_light_client_update(
		RuntimeOrigin::signed(ALICE),
		GOERLI_CHAIN,
		updates[1].clone()
	));

	submit_and_check_execution_headers(
		pallet_eth2_light_client::mock::RuntimeOrigin::signed(ALICE),
		GOERLI_CHAIN,
		headers[0].iter().skip(1).rev().collect(),
	);

	let mut header = headers[0][1].clone();
	let _block_hash = pallet_eth2_light_client::FinalizedExecutionBlocks::<Test>::get(
		GOERLI_CHAIN,
		header.number,
	);

	/*

	TEST Proof genereated using : 
	curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getProof","params":["0xdac17f958d2ee523a2206206994597c13d831ec7",["0x9c7fca54b386399991ce2d6f6fbfc3879e4204c469d179ec0bba12523ed3d44c"],"latest"],"id":1}' -H "Content-type:application/json" https://docs-demo.quiknode.pro/


	Response :
	{
		"jsonrpc": "2.0",
		"id": 1,
		"result": {
			"address": "0xdac17f958d2ee523a2206206994597c13d831ec7",
			"accountProof": [
			"0xf90211a0c2a926db77c30554ca4188bc43ad2871504ded25c5408bd2ce7fa6bfa2f63e45a0e7620cf176adf59e6f113c33f24d52eff5ec50e79d892fae84bb0a3b47631332a0f0a2f55a8a2503cec808e83f4c6abf938159e53eb21d72bf4018dea23079d6a6a0e5e619b66ac727376427e7d9589d2f435c2d2b85108552829bff0a6a422c7fa4a03313be821370260868d65cff1e4818e66c3d67b0b68e772034a33402ac92d6eda0e329be13e1d8a5a5f4699793f712a111b7829aba4aedc89718a5f824019ccf31a0d6397a30de4920969f0e4fae9bf3a245dd0057731ac3f17d4a60dafe6bfb9b83a0f7bd36dab236cf36cceb8f1e67f6eb9f58d4f16f2979312be44e150c74e35f73a06b9609788fe8e436766001343bd6c967e8ba214761736f543c59b953d962f228a08f38babf3c7f8d356f551f82744120e2c91055c401d80c8194bbaf0f0477fa5ea0e4b6a23ab3ee2c8c9102fd50391891134acb91d11e68767462b3e6ea62362dd9a0ad06b7925856cbd3c7d415a7bf10d011d50173f896aeabd64269b165bc931205a0dd5749e6c60c804992819f4c0d52537e8a21c78b41bb0c326ae2770407a327a3a07be6bd349b0515c51341cee029e6ccf2dd0baaaa96432d30f6e03c4f548e3cefa0978b3236c97cee751e81b7eaa37f3bd31ad7a1ba17c94eb4a807bfda3b4053f0a0cba678ef3b170515a6cf0e1c67b7d176e9a741d8f29dac405346701b2de6d3d580",
			"0xf90211a031eb575cf02861f834492920025ecff8f6991c938d2fc22ed7af020dea760143a0192e832274d19b4a6dd1036993f3e3b365f42002f2759761f8764545457562c7a0cb6a9983bf5f8217f6740303de48d61333c200b6cffc54adf3377ab479c957cda025e64e46e9dd3a68f6479ceade7acb99a14afb2b4a62f55e39c7342e66c810b3a0ff9fa814e8ac7588e242050612b4195e3df6aeffd37536d35dcd87afa4c59ab2a046f73831faae695e1695e37b692d7fd78a0654ffe90bc3fb0bfa20bb5fa121b2a03b4618d5f5e27ac834cee82bb65d51979607d3dd9fcff63772f3faef3e7a92a5a0c2760b5fe6fd428866459fa1441ac606b999696abc639bd330e4f4287929109da00108137d542600c30c86c2ca3f397b2c25e3b93da23ea7e0c14d9bea4bb0b6b7a03d6c67ce964d78dbe955ce003e4eed34665b372c44f64e2d2c4697687ca5a5baa04ce41e2e39de195f61424f1615c0a8d27cf755b2c19c1b151d5228e741f8eb05a04f15a39af0d63498812d57065c4395cc8cfca35f64381193b26448d9a53bcdf8a0507a4526a5659270ce64e53c84d137b0178f67b9fce8cb3b675c98f88684aa3da07f23cd64ffde9ea687911c105ea801c99f817e385bab530d983cda8ecfac3737a0f4f8c45f8ff5b475bd61291ff19e56476d4564f0e642ef4fc7805de64e754cc3a0346d5fc008f70d13a34aed0049ef4cad4578b8484a41922b957cef2305f1800b80",
			"0xf90211a00cba5c045cc10db57ea371d4d2c669f3f6a1bf2a545836532efd62be106c4353a0742f6cd31579493f71e25c473daa054fd1e0483f874f882ff3e57ab678bc4443a04bd3013503dd4ba1c6072abbfa7dfb8cbef81abb367af264db1cfd1491ac56eba0f769c3567edf7ea8924c06a5766e1e9d557807bd52cc8950afa70e63eda13289a00125516e388fdf50c056f855f60b006dce2bcbce88905275e606c76e6c1e6da1a058ae64f3af6c5e2dcd6ebe69bfe9b5a88e864e8d80566acf866560fe0dc3c91ea0f9530293e671e8aea86f364a0c59edad00faf3b12ef6040c552555f6ec1471f6a0aef1ff96894b24974b3156c09b5f3cab54a831e64506a7cde2ac21f28e666f21a07d1d8a45f504a0e3dbf009351dc4877053b043d8d4669d75d50dabfb81dc7458a08938f65d6d8deb0a2c133f95fe7362c5003598eaed71daaf8f97f695c148fab6a0842e8a96287d6b553cc9d9b67440dcdcbd71054d2df4a4b575219f90e35b9c63a000d29ee181a47455b707e688e00a078630807d2a10a136d129387eeb01b211d0a03a81ecc801a34ecee21214093fa27502bb744e319c0e316470fd304b89ae7a45a0bf650c2bc6fac8d1fc4dba96d5a2fa010ea6ed375cec8125c78b227f9a142663a02214f5bb331de8eca5459e94a2cf54367f2c07470bf6f436b7238355273eeb7ba04899b57adc33deae796b9d09fb2be90987784fd60b9dd816dc7425eca25ecadb80",
			"0xf90211a049da47bf30e55ebc281d1144e5c304a323bf88939dd7f87a34c16c8cc46ad8b9a0e31b5e352c2a964cdbb198ea968190ebe1712ab6eb825bfb8fa329a214a0c25ba018754dfb7248e2f9bef029d618fb8cbe59ba11f79d611896633cba08df9a6be2a0c53e7bf169ce1bbe288501c09f08f8500975db51f2c31af46c459d88ca6c2de0a0ff4d7190c3ad4065e15b8e987b6f17edf50bdcc188a4451a8c38495073abed08a0365bb514d60a7b24d774faa728597b27edaf1f9d374e0caa6568df61cb60c29ba03a32a541494c0c693e0ce79bee8546f53d7fc1799b2bdaeba8164a74b06fd9caa0adb98b57f989d3b21600eb6b361f5b55bd963acf6b34449cf5b4b886aeafc5fca06f9cb11a1fa639940ff1aee5d1882c97ecfae2eab77ca5a75b70d8db6671f231a0f652dcb49419e4a9d84d658e8654d284adf87831844639a30792696af4b0df70a0bd363e2627ce91e36ab069b87b6af6c6aa13d0c1a682bab32ea050c6a7f49e57a031fe749b68414638246d6e51bf9c0cb119eab90e8aa27ad5892c6cdb348393dfa0c348b463e446f6fc9fa6d8b03ad13ed942e5690dcc0cf266919175d313fc58c8a0d548389e899fbc2354d3aca87595bafa110588a38f522521996c5a53dfbc19eaa0ea8924bc581eaa70ddd3e49e4021d5c3fbc7d10a059a9054fc4d0e6f8e37f68da0ff96d3e8a5b7f0a413bf0b2331fc3bb217da8bfc9a6a87d1cc6fe1fb5e83347880",
			"0xf90211a044adc0afecd6d92c4ba36df52b3dccb4f0bc7fbb4a840a79dc894db60f9d72b8a08551eab40e5845a37eed3f83c969c45be34c53cc20fa1f79eaf51616c1839966a0aaf2214a29e36d0f0fe67d2a9731cd180f12ea7059c04c25228fb282e396d48fa053987300dec44d9a39c541e12f9b8775c4277d9f688126df193de9cc0b41af1ea074025812caf1114aa4974f1757e85399d320c2b9a28da0ee46e52191730d4628a0612a17956e3d5b37a61717914d4e37f8b10446b69c195b2e63ccf9f8ca045f4ca0971018fefad08f1709cd4dc9cfcf60046f73b611582da5b41ce99793496ac053a0fa59c1774b98d21e11d6473a62650cda4845473babcf58e30e4711da1617e02da0245d178a063c7f0befdb8d13d582fa0219c83e997db580d510f174efd14e800ca0e585f68eda47a9b71ab42c1b7b28a4269c632d93b1cfac7a19f538679f8a903da01bb37af0e639fbbda3f59acc1536fbc34b69d8a4f2cfde842901c81340d973bda0f5c12a68135c6cb923499894c831f1bdfc5cacaa0ad923cdaacb6679dec09d20a03b6942ccc6d90fba4d9413c25997669c386eb1e23d44268630de24ea6cea1762a0866eb94dd3596bd96d34f0e3d69fcea5e5f88a2df91e913ac3a752f94240bd53a0f8485d53af73e002c2d61f809486786ca236ad833b8e34210737f63f5e667da2a0306fa22286407df0af352b148c7d79e2a8a625205c560aba57bef78d6844ca4880",
			"0xf90211a0e31b4c19fcc4fcb854b8e624b318ad21b022c57c58e30f80f27bcd9fe4e61649a0950522557eb40bafbd082f0f5cc4be3bcdcb7f80c14eee43afd2bcd01f8d5137a05344aa1c9ca2e3e56bf98fd718ec43728578d148e1967fbaf8bf17a2a073a0bda011a2f9312c3308640a0d6ceeae218747290f23806067456da1d444c65abae437a0b3097a108bfce79af6699da4ae3003cd4929f0b4576aad655c31cb725bde84c7a0bad5e9a43b6d7c892edfa96dfd70f230f3529ecbba8ffe2bec5a2136f5157dd4a0cad699d82f34775cb6c142759debd1b008248172c572522769f6e3b3885a6595a0861cbbbafc57e5a50213192938198fcd5a5f52cc40d65843e561f2aea128b1dea09aa229b179290efb4a6b52687b928c0ab96962acf989d59de2fde5a1a657142ca062a88a2900544dc76a32255a6b2b2a2eef8fa68279700c00adc7508286702552a00b3757b624f3e65e3cadbd9b61e092af2f6086fe846ac6ed51a2f0261d21b475a0b7d528fc41c8fdc8ea18c6e7d0099270c777ec1403cf879d1f5134bdc12a6c6ca062a0b052298c12a244f472f1eec5f5d387da0760f654d348de7439855781b655a0c753911ab60857ee26c554b1f2caff31e90299612e79359b0cda69d6c271b7d4a0252b09f4c50f744dc6274f41f22954731441a1945c4979301bfbb5106b84ab11a08bd2b242e992653fa60521d04209d0f948548de03ed9d063f6c847212da606f480",
			"0xf90191a0fd611268e9e8751b202da3e2b9cd29276a2b9ad8a1648d0214bf3bcb09c7ab7d80a08537f2e248702a6ae2a57e9110a5740f5772c876389739ac90debd6a0692713ea00b3a26a05b5494fb3ff6f0b3897688a5581066b20b07ebab9252d169d928717fa0a9a54d84976d134d6dba06a65064c7f3a964a75947d452db6f6bb4b6c47b43aaa01e2a1ed3d1572b872bbf09ee44d2ed737da31f01de3c0f4b4e1f046740066461a0fd0510855b792a3178d5c28befdd97b3c7679a810034dc60fa7b2fb24ef10cb0a0774a01a624cb14a50d17f2fe4b7ae6af8a67bbb029177ccc3dd729a734484d3ea06fe8e5a06cc831e80c924a68f53841baada661d93decab89193ce5f43a921d69a0c8d71dd13d2806e2865a5c2cfa447f626471bf0b66182a8fd07230434e1cad2680a0e9864fdfaf3693b2602f56cd938ccd494b8634b1f91800ef02203a3609ca4c21a0c69d174ad6b6e58b0bd05914352839ec60915cd066dd2bee2a48016139687f21a0513dd5514fd6bad56871711441d38de2821cc6913cb192416b0385f025650731808080",
			"0xf8669d3802a763f7db875346d03fbf86f137de55814b191c069e721f47474733b846f8440101a026ef925110ffd83caba0e1df30bf83763582fb8bafa28751cb86610cb1fc616ca0b44fb4e949d0f78f87f79ee46428f23a2a5713ce6fc6e0beb3dda78c2ac1ea55"
			],
			"balance": "0x1",
			"codeHash": "0xb44fb4e949d0f78f87f79ee46428f23a2a5713ce6fc6e0beb3dda78c2ac1ea55",
			"nonce": "0x1",
			"storageHash": "0x26ef925110ffd83caba0e1df30bf83763582fb8bafa28751cb86610cb1fc616c",
			"storageProof": [
			{
				"key": "0x9c7fca54b386399991ce2d6f6fbfc3879e4204c469d179ec0bba12523ed3d44c",
				"value": "0xf4240",
				"proof": [
				"0xf90211a0dd11ca5cc63530a39e4604455988257545e84497d17b996ab404b1aadf7f9a15a00db77e0413a97cd6b319f56c2b3fa05f77b54136816a4a048c03d290eac9f2b4a02a92329056a90a0f98ddb6a2cbf1e4896b23a990158bb01876489eecce5ccb79a0c165d5c24ce7a9526d1fbd7962fade0f0d6e296e3002a9406606a86854ff134ba0f0eaf8d99ff84925ec96bf56f88fe08dcd1dfcc0b166b979537896e3318e161ea0218b085b3878cb2dceccf39a1bef0dc79d0f5ccf3348f526a2071b0b5ac5824fa0f1717e9528b7fae4e1aba74d9860842493c26407d5894bda2322769bc8ce535ea04201b6d063db1cb27e182eabe6b9aa5f4b97504e8af852b94e00086fc27f2132a081db47bac915eb4d0ae7aaed4f0866f7caf7b9c398ad5e39d8e49ab7a90584f7a089d69983486a96e98357c49842324be9954a270aec09d39e2b251f77c04a79bda014e2b2c9df7c8934e972db6509326a9497c40b87da4ed1970ebe8c5bb4c7aadda06b175efa62d7b05624f057a9592a91d4ed651ad5aee841647ad26e71d9db91b8a047a645c2ec0c806a86beb548980a7e7777aeb063a2e859f1160be73f913527e0a0db74c43bfd799a37383afea2359153e5941cd037883904efd333adc7cc2da972a0aa31736c8248a54c66f7764b7508d94684bd241346511362be33491bed1f5874a0889cd8fe4379031b7de62ade494d367e33f1fe3f6c2a1c438a090a36cd8c04af80",
				"0xf90211a07e1cf7eca85b785eeef74586b8a64801663dfc210477bd50e1491fca3f67f7fea0820c22217cebb924a4380526f2185cc25bb39cf75306e946e409680e0091a864a082a08c4a2c4cc0da12622911e2c5054460a0370ae6839f5804f24987d237e5b4a0a992425aad8ece0f639b2ec37bb25a77b1f264ad690800d4221047086e76ad40a008976dfe219bcabf653ebe1d93017db0ab22d018c22b1df4bc8cf9ee9af8196ca0056a637558b1495be7f638fe3338f22193cea8e52378eddabe074c13b62f72eaa0631ac5a9722a282bf647541b98376dfd51f4db7f566c531c777938bced63740fa0d05a199e901253efdea1e6674b6855faaf9cddffca57b7f3bd92cc92651b84eaa0982e54f2a32023e0fc675752897c4155fdbe365c25d558113e949695512b658aa027f492909eae1b40f95f535149fb4d1d4cd8a7334dd8644f72b2562237323cdda02587fa7ad446ee3cdf9c3f6988d38dbe5f86678d666db1883f36e3cdcc5083d6a016629d1c1438d16e110a90af213b95b65a9ccdf36289575f5f7aedbb6f2a0b4aa0f398d24fe24daf2111ea6bfcff18d3f9d562232fd75e867062ccbb57fc3704c3a0f308c668c44e175bb0e246cf1451d67744d67756e07fd1528c208003f424b3eca0bf8ed7e3df80e75333676944b4e404cc3358d6356f42a40a0b4926778a2447a6a0501a7c65ed401d90e0616a0829eb1000acd3196d0fc5d76c15d78743a486134980",
				"0xf90211a0e5ecd7b8049cde14e7cc4a4db796825099124e2ffa51f8eafa690d9c880896f6a0fefc532bb1f1a5b9c9b0f87318acd6b9f9a0fe4e2eb8290cd21fd647b659641ba0268540a5a8db62078e6418034ff6895c278e4068fbe25098497626e191dc7a40a058ef6c0d7b60497aeb0a5cc91a54a947dd534a9dc14e1e30f3f054dec4ec1431a046307aa62e9d962ed7cd1695db67e06cb9a5a3ffb85a59d57892edb2d1c55b70a0bb198c4bb694d4c38445c55fceb186d04cec94464b404a1e3ed1372425cf0d82a06b9c2e9550e244014d105d45590a9685e889fd78ffe0e7776ecc17bf90932d5fa0410cd0fb3c89caace709ee28b855b96d2becb321272bfa6bc82e50ddb8a78c84a00e9d5f46fc1c0b860b093bc5173fa782763b7a16599066616d306e1b6ebec3d2a05d91559ebc497d4b723b653362aeeba8e36a6030394ed0868c44ae6a8c83e86ca090d9a92f79afb00e97f4765082aac68a02259941a7697d21bf98afd914ea4770a07b44435b87744d01a192cd79b44ee26a8cd3be86dbf397e630ca65b6380bce96a0fa3fa0759b0fd88ed22cc79ff9faa62248886a86b734197c308dc2c19877bd6fa0cc6e8817bc7131936f514342eb15f294937c6353f125c78c4a8a9eb478deabc1a0962cb80c5d3e53f795ad84a9e7112f77a22b7102f92ed90462a40bc89a56c48aa05be624dd8e997398652ee042e9e21516ecfd8b0d13c745107b6a646f2e2bfa1180",
				"0xf90211a017734b6b3e09a5a02f73d074361964f601a633cebe5d3941d50e2d8a7d648343a037612ac06309796493fc97503084fbb50c4fd9f7f04774b635770169ec70997ba0ddf19604b69f56fa5f545dcba901408c7155440db8cff470caec4f818283065ea039ad9ed144772ddb55a3fe624474130ad19d29333c402e4d8eb731aa2a3f8afda01a2517c3c5d814a784dc030ae3c0f842098bac5ce6c0c2d00e243d978f4e02f6a03fb6488d819ba928f66ea218b9274afe4170918a4507edcd9f1ad122b87b4548a009b158ec91c4c9493d630d1d22f59a433fe96d9c2d05b7ab0e40b5c85a7f4d70a023f35c1b3d39406a7c13f6c1676b8b184fc5f2e1a96ac6df943b3e67138cee8ea096216fe8f84885eda7972880194086f6065a67e0ee3af1111d02f62cf675b3d8a048e73745232e44dc11332bc4d44ddbc39b60d40e25217754b8f0d282d7e8fedca064cb4b394d0e0ed4e28a2cf9dd8409e0fb48017783148bd654b647415813a242a01bd63e42651f875bd4bfef0b7ea304bdeecbc9db45866450e615778084cf89cda003fe4bd31a555d9ed909c02c42137c28c8ae05d9979c536336731653811bea54a05c9c5214f6c348f622b8269c16cd83951db7abc9a723b719123451a7d8b0ec3ea0c86b70a761cf23fd81148552ef1fe096c09ef5dbbda8779f63b5a1a44eb8250ca0c29aff503291e68d4a1579fecd36704c582c0741fe82109199066cb6fb32ecac80",
				"0xf90211a0188286f3f11bcb771c556b5ee07c8d759c9ef472892ddc0ae9619eb3b0889405a02fd8aa6d06cb5dd3e478b8edb5a4278db6dd51f77fdf548ba0a35952d4eaeb2ca0ba7c8b8f3062acde8c7e125aded534b3a9fb54f1edc1c9a0470a53228f3f936da0f419c7c629ba79821692d00188509ceb2d41652ddfd49700241e3848e212e0caa09d8400a6f1864fdd4291104bd571cb9aca73e17f51071c8c3be9e5092a43cddba00d2ed1eaa16e5f3a5d77be1ca0ffa5a4eca19d7b952360d743ef238c93db2394a0cd13247f88437a0d238ad4684d1b95a5029a5b6673ba7ab6009f4c4a877dcb8fa0775de0434fd6f996a8e85206dd8b73de1b17bfabd5fe4ef969b44befc332a6a9a0032fea4b8402fbea7440298d655ab008898e99f843051c62adcec2db7e7651cda0607562c790d00374d1b5d28f52443258930b2dba280b34f8830819b08a7f4488a0752f7ef755ad535a4d5e91f2fdc503ac73a22e847da543b76523c44052a21a5aa010cfe1d7c9fec8afb53ee56d0162c87c1c961e6f0b0a74d1755435441a16fd86a09874f8a6b2655bbef86270ce2c28cdc5ff9bd8e5d26b18264a842f0c76ae2bcca0a9a1fc0e7fde4e76ec4f86fdd9c9bb7c199f3ca482c051b99e23735760e52c5da05df600c867e91bfd103a50dd99c386cf7dfe0a9d58c60eace62b658ce892ec40a06400d4255d478843e8e19f708bf9238b9eb4159308210390ebd0ec1596479a4280",
				"0xf90151a0f82991602558fd2728526111b928929bd319331752c26327b0632b1b2ebd335ea0f8f4a40c0d6f7e2721b2591668fa7ecdab457ed9b9ed119206ec7dce992543f6a060cccc874582346f9b00cf5f5f173021b7e6a261cc87f42e6554a4b5655a35d88080a068bbe85c9cf02d44cf904c054efd76d95733b70adc211531a2509ec04667ce92a0d8f69702bd1160675e32a9592da63480c4692c9ef7def82735bd611141f7ab58808080a061c279e48054785f26a0293e5d71e6dbac5d472f1e31eb1c3ba21de315f3207ca0fb165f45d07bdcdb53ebce926b1d88db216f47191c2613ebebb1cc6db0c2ff68a01a1758302cf86b012eff63c8176a71d9f8e142da3294d29befc0848275638a29a06c1f0b82f78e5cc8d7d9d464c062d538898a8d641e362846b6e14bdc565407e180a01c97e59e8a0ec38b294181d332960bb410bfa5a55431cac195c72df387a7291780",
				"0xf851a070fad6ec88f6bf25b4897a0bbf9cce66c5b37d2e259775cb4403f97d9e179fef80808080808080808080a0e1b81a62bde15f0f24034cad0718145079a65129120771cb78f1f1dbad60980b8080808080",
				"0xe39d3b9ee75012d399b185c759df602f3472b9d09b622a5ba5f48d3edb39fc84830f4240"
				]
			}
			]
		}
	}
	*/

	let hex = "26ef925110ffd83caba0e1df30bf83763582fb8bafa28751cb86610cb1fc616c";
	header.hash = Some(eth_types::H256(array_bytes::hex_n_into::<H256, 32>(hex).unwrap()));

	let key : Vec<u8> = "9c7fca54b386399991ce2d6f6fbfc3879e4204c469d179ec0bba12523ed3d44c".as_bytes().to_vec();
	let proof : Vec<Vec<u8>> = vec![
            "f90211a0101e7ce14617e3b6d6fdba3fadb2725614adf6958a0a94c80f850663e3e11b27a0c7fae9be4cd2575a8d72442e649bd8e8e054a107085600e75170833814b012e3a03e60da2bf49fe4e71073808fc95016bb0dc5bf0103fcc01fd425cde6333e373ca0e48c50d9e1323361e1ddfb0246f9e8094114f480630743d5d20f8dc8f2c36c1aa0bb842ad10edf0a152bbda602192445e1ac656cee50d57aac4ed92f7f2a624daba08f7e00bbebf5dd7c0e951c36c0308be76a71381f6aecf61b2b95f28483aeca6fa09691f7468c11acc8fcc60034a0c1a054e551b79fbb360e9edf53359cc297c71ca0355e7b555a9e0290607cca37367afde2aa59e71585dc5459b54b3c23f456e024a0f141f94285bdcd0658736ba8d3971b41e311d7bc4d69163e7a2f47a6f1544439a0f688b9b16c405efaecdb7ee97d77f07cc92d3597f6c678b37c3d10ecd190d1e4a06341f0935b4e55160b8a4cd990b6da62fc17dc895868ff2b24b9d0e2d51e6b5ba060ba1049e44915d9884d5297737e8f50926a2fe1333908ed4408ce7985b62ac9a0fd5d6445fd7f584879695bdb9aecca3bc3b32cf6fffd823c59b1233b34458db5a03e16aa29f3fa3d88cd43d553029bf5b2a6a8caccc458f0485248b02d5d350858a083e9f5b325bd2f46b57278f76167340dc62f36d03cf0c283ede85cfd2a7b5879a078f5f3a00d5e4c052c38f90fe95d0003ffc887b352c6543acb4c1dae7e9c4d2080".as_bytes().to_vec(),
            "f90211a06b3e9b4c4a9f3b35235f041dc87d102c0d61210a596d215247a6f0e21c112e3fa0ac3d3591317f3a9149973cd750449e4647f237a0e6c6285e52baa755725c50b6a0bc77317fdb363ada827f141e272b1d3887c2c44d1994529ed5a2c851f506c43fa0a992425aad8ece0f639b2ec37bb25a77b1f264ad690800d4221047086e76ad40a0624666b28e7f9fd208d44c7b8a1e370c9d0eac2dd4d120508469806e99bc562ba052a6d2a2ec167ae0a1fe49f2356e3093a90cc7d3296236f35115300e76cda21ca05d5c3e4c9177224ed73c98af3713b6578f182c2301a0d049b843595882da3cb2a0aaba7adc9c55d6a39d6d9a15e06703087aaa4315555781317a992c466b22a108a0982e54f2a32023e0fc675752897c4155fdbe365c25d558113e949695512b658aa08188bf5d4428921dc77b8a4e6f5e3c84d01cc1f0e5eacbe0b65a2c8bc4937c2fa0b95bc293ed00483fa4457e975a48c49654e498c0881e0a93d25873b6818e8933a0328e0d6cfb455ad4bf2b3500aed9250e7961e8877fe730f36efa3cc3a719a7f3a020430947c0b5a7d6e69d8d8ec8d6d8b0b45d8d737e51d9f92621fd29445a9a39a06c0d4fe1dd9296edfe75372da6c6d1f889df4f0111e7cf44aa62d27601860eaca0ba3cb0c062ac33f7291ba287426ddd00116f660dbecd2962c522bc3181d19576a0501a7c65ed401d90e0616a0829eb1000acd3196d0fc5d76c15d78743a486134980".as_bytes().to_vec(),
			"f90211a0e5ecd7b8049cde14e7cc4a4db796825099124e2ffa51f8eafa690d9c880896f6a0fefc532bb1f1a5b9c9b0f87318acd6b9f9a0fe4e2eb8290cd21fd647b659641ba0268540a5a8db62078e6418034ff6895c278e4068fbe25098497626e191dc7a40a058ef6c0d7b60497aeb0a5cc91a54a947dd534a9dc14e1e30f3f054dec4ec1431a046307aa62e9d962ed7cd1695db67e06cb9a5a3ffb85a59d57892edb2d1c55b70a0bb198c4bb694d4c38445c55fceb186d04cec94464b404a1e3ed1372425cf0d82a06b9c2e9550e244014d105d45590a9685e889fd78ffe0e7776ecc17bf90932d5fa0410cd0fb3c89caace709ee28b855b96d2becb321272bfa6bc82e50ddb8a78c84a00e9d5f46fc1c0b860b093bc5173fa782763b7a16599066616d306e1b6ebec3d2a05d91559ebc497d4b723b653362aeeba8e36a6030394ed0868c44ae6a8c83e86ca090d9a92f79afb00e97f4765082aac68a02259941a7697d21bf98afd914ea4770a07b44435b87744d01a192cd79b44ee26a8cd3be86dbf397e630ca65b6380bce96a0fa3fa0759b0fd88ed22cc79ff9faa62248886a86b734197c308dc2c19877bd6fa0cc6e8817bc7131936f514342eb15f294937c6353f125c78c4a8a9eb478deabc1a0962cb80c5d3e53f795ad84a9e7112f77a22b7102f92ed90462a40bc89a56c48aa05be624dd8e997398652ee042e9e21516ecfd8b0d13c745107b6a646f2e2bfa1180".as_bytes().to_vec(),
			"f90211a017734b6b3e09a5a02f73d074361964f601a633cebe5d3941d50e2d8a7d648343a037612ac06309796493fc97503084fbb50c4fd9f7f04774b635770169ec70997ba0ddf19604b69f56fa5f545dcba901408c7155440db8cff470caec4f818283065ea039ad9ed144772ddb55a3fe624474130ad19d29333c402e4d8eb731aa2a3f8afda01a2517c3c5d814a784dc030ae3c0f842098bac5ce6c0c2d00e243d978f4e02f6a03fb6488d819ba928f66ea218b9274afe4170918a4507edcd9f1ad122b87b4548a009b158ec91c4c9493d630d1d22f59a433fe96d9c2d05b7ab0e40b5c85a7f4d70a023f35c1b3d39406a7c13f6c1676b8b184fc5f2e1a96ac6df943b3e67138cee8ea096216fe8f84885eda7972880194086f6065a67e0ee3af1111d02f62cf675b3d8a048e73745232e44dc11332bc4d44ddbc39b60d40e25217754b8f0d282d7e8fedca064cb4b394d0e0ed4e28a2cf9dd8409e0fb48017783148bd654b647415813a242a01bd63e42651f875bd4bfef0b7ea304bdeecbc9db45866450e615778084cf89cda003fe4bd31a555d9ed909c02c42137c28c8ae05d9979c536336731653811bea54a05c9c5214f6c348f622b8269c16cd83951db7abc9a723b719123451a7d8b0ec3ea0c86b70a761cf23fd81148552ef1fe096c09ef5dbbda8779f63b5a1a44eb8250ca0c29aff503291e68d4a1579fecd36704c582c0741fe82109199066cb6fb32ecac80".as_bytes().to_vec(),
			"f90211a0188286f3f11bcb771c556b5ee07c8d759c9ef472892ddc0ae9619eb3b0889405a02fd8aa6d06cb5dd3e478b8edb5a4278db6dd51f77fdf548ba0a35952d4eaeb2ca0ba7c8b8f3062acde8c7e125aded534b3a9fb54f1edc1c9a0470a53228f3f936da0f419c7c629ba79821692d00188509ceb2d41652ddfd49700241e3848e212e0caa09d8400a6f1864fdd4291104bd571cb9aca73e17f51071c8c3be9e5092a43cddba00d2ed1eaa16e5f3a5d77be1ca0ffa5a4eca19d7b952360d743ef238c93db2394a0cd13247f88437a0d238ad4684d1b95a5029a5b6673ba7ab6009f4c4a877dcb8fa0775de0434fd6f996a8e85206dd8b73de1b17bfabd5fe4ef969b44befc332a6a9a0032fea4b8402fbea7440298d655ab008898e99f843051c62adcec2db7e7651cda0607562c790d00374d1b5d28f52443258930b2dba280b34f8830819b08a7f4488a0752f7ef755ad535a4d5e91f2fdc503ac73a22e847da543b76523c44052a21a5aa010cfe1d7c9fec8afb53ee56d0162c87c1c961e6f0b0a74d1755435441a16fd86a09874f8a6b2655bbef86270ce2c28cdc5ff9bd8e5d26b18264a842f0c76ae2bcca0a9a1fc0e7fde4e76ec4f86fdd9c9bb7c199f3ca482c051b99e23735760e52c5da05df600c867e91bfd103a50dd99c386cf7dfe0a9d58c60eace62b658ce892ec40a06400d4255d478843e8e19f708bf9238b9eb4159308210390ebd0ec1596479a4280".as_bytes().to_vec(),
			"f90151a0f82991602558fd2728526111b928929bd319331752c26327b0632b1b2ebd335ea0f8f4a40c0d6f7e2721b2591668fa7ecdab457ed9b9ed119206ec7dce992543f6a060cccc874582346f9b00cf5f5f173021b7e6a261cc87f42e6554a4b5655a35d88080a068bbe85c9cf02d44cf904c054efd76d95733b70adc211531a2509ec04667ce92a0d8f69702bd1160675e32a9592da63480c4692c9ef7def82735bd611141f7ab58808080a061c279e48054785f26a0293e5d71e6dbac5d472f1e31eb1c3ba21de315f3207ca0fb165f45d07bdcdb53ebce926b1d88db216f47191c2613ebebb1cc6db0c2ff68a01a1758302cf86b012eff63c8176a71d9f8e142da3294d29befc0848275638a29a06c1f0b82f78e5cc8d7d9d464c062d538898a8d641e362846b6e14bdc565407e180a01c97e59e8a0ec38b294181d332960bb410bfa5a55431cac195c72df387a7291780".as_bytes().to_vec(),
			"f851a070fad6ec88f6bf25b4897a0bbf9cce66c5b37d2e259775cb4403f97d9e179fef80808080808080808080a0e1b81a62bde15f0f24034cad0718145079a65129120771cb78f1f1dbad60980b8080808080".as_bytes().to_vec(),
			"e39d3b9ee75012d399b185c759df602f3472b9d09b622a5ba5f48d3edb39fc84830f4240".as_bytes().to_vec()
			];

	// Call the function and check the result
	let _result = <pallet_light_verifier::Pallet<Test> as ProofVerifier>::verify_storage_proof(header, key, proof).unwrap();

});
}
