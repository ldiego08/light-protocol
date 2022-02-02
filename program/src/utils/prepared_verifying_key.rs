use ark_ff::biginteger::BigInteger256;
use ark_ff::QuadExtField;


pub fn get_alpha_g1_0() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10802560400167329898,4427831596641847289, 8446202072820133997, 125567519208212759])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11751420931253826222,8710609348661928048, 4579119150976691833, 2516190483927819264])), false
	)
}

pub fn get_beta_g2_0() -> ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters>::new(
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13957427368410267022,6101563348155213896, 15595705088219937741, 2977830004324324263])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7792285146718175302,14431480397106960593, 8891978072743389075, 172671505553152981]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([383450373217395126,3227181728795496973, 4495371084454688693, 522152305940961108])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18054170617781614761,18267147348858885998, 3733588098824404793, 644943871665481251]))
		),
		false
	)
}

pub fn get_gamma_g2_0() -> ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters>::new(
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10269251484633538598,15918845024527909234, 18138289588161026783, 1825990028691918907])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12660871435976991040,6936631231174072516, 714191060563144582, 1512910971262892907]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7034053747528165878,18338607757778656120, 18419188534790028798, 2953656481336934918])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7208393106848765678,15877432936589245627, 6195041853444001910, 983087530859390082]))
		),
		false
	)
}

pub fn get_delta_g2_0() -> ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters>::new(
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16689169217387814863,16596113716224990498, 102212514645860935, 1744521945055362426])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1658600239166004965,7261792145891956298, 534723100432490517, 2204654943156592269]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4028522621512790858,15596766072332154861, 10280015726270376324, 1772008745412117699])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1519445928371812555,5713975852492434186, 15869210945216942841, 2813412174874088513]))
		),
		false
	)
}

pub fn get_gamma_abc_g1_0() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17618785193146642264,9383713381980625880, 253017162204341459, 1196257564496381844])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4136265497527796166,492708975402171935, 10362282601807913544, 1879513740747558768])), false
	)
}

pub fn get_gamma_abc_g1_1() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8799300210269103075,6859248111665093027, 4684087672800459695, 792612450649726682])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10309522592425242966,2632158090661150300, 14409109425237544668, 2125905641635682551])), false
	)
}

pub fn get_gamma_abc_g1_2() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16915543079351154767,6141721250254319183, 14988030507003306625, 2805202618987747944])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4038902334487096515,11873850827858013514, 13421630452499854258, 1966821340233670046])), false
	)
}

pub fn get_gamma_abc_g1_3() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12982520277415699281,5875622664846720978, 10660726589334387513, 1342783423397380484])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16962330855307397100,17866149351499587591, 5702845399201481103, 1776736447652320268])), false
	)
}

pub fn get_gamma_abc_g1_4() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6162433087188172533,8237577102601834332, 5593334675971803592, 654397904870583957])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6822969639868858460,13155117230846832092, 14332696208349170591, 2431197911537611919])), false
	)
}

pub fn get_gamma_abc_g1_5() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2729869805717678607,2015545140788036236, 4890603898372131845, 1611363774002898469])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3469799044006686684,653098240645113851, 4191512383709540109, 3386855162436160834])), false
	)
}

pub fn get_gamma_abc_g1_6() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13839798083163325096,15022027847197962587, 5771637217969387193, 1244991285899390938])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11912177932026842940,12675451820257557806, 905757285286250263, 2998859187180542534])), false
	)
}

pub fn get_gamma_abc_g1_7() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2940715471137407653,1583527404047598016, 228856748249008244, 989323264464852084])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10944747435637216939,1709482276123345628, 18408935755796419161, 2962467750302232271])), false
	)
}

pub fn get_gamma_g2_neg_pc_0() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9735490623776675493,7313347297369877603, 5110441044595811232, 2420314695870899172])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14416786213697531356,13308121799468939638, 12390083706888003821, 1966175061718780164]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4129257347520110928,13798226624051452651, 4825670390762580777, 1989277302133421735])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11104699749248547751,10435997551076758402, 17853110753348405340, 3361471515497012039]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10169789301848189331,16016180067228186549, 17334750741304028879, 2228788662616803775])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15172957284714629703,13417154794643176123, 3196086454825695542, 2093866205601446741]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_1() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6373452946747570674,10270768430483208834, 11341147745087012459, 3157052191146643204])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([602066187160159699,1578931260951444474, 1587541677266892445, 1992373586887236310]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2035058501502628319,14930432017151590998, 15355551583521351086, 469587794589787657])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13631719073532767446,2065158137318837312, 5775538604822855962, 2383111915651801787]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17544310453790923341,14459545592572037104, 12200103993180316021, 2090533022732391846])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8054743094808658598,15345477660971473493, 15443796689430031587, 1806066076678295575]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_2() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1307490507467590467,15702387221270537235, 5269129970681753992, 28002378715318771])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4660357668607340467,16578489089999929478, 16748828149682735846, 739979399064110919]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4578782715327068294,8628960991187287885, 16091835164139194461, 852554802780718793])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12096117391014189539,18239444815601499298, 7233070439485440435, 3194275071475042713]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5827190618476342597,12470106628583867316, 17200718410803934706, 550783932675933241])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15647736919678071305,12053702395563955525, 4650097433064156528, 2812014987399368919]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_3() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1836789663054811950,11512141512792473873, 1478746144118729173, 2923691560477017483])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12829111010025739515,7250912810722826978, 8129015919716064956, 1557336640775108833]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16722324985467231953,3455514969581926786, 5435134192097375645, 445144570921449663])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17552173304110772017,6392449665810583181, 15317764502253575963, 1986700432257093656]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5236977799088141757,16542968308152612384, 11217115100283931318, 2197361333128902643])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10433619994713991449,5468729870700106286, 13731897016295146488, 3282210527109190724]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_4() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4097647836534978155,16211718821644978109, 12543988062359842685, 531950869723565272])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3732584527186327026,11009433307933628691, 9906125292583317817, 922464594331819969]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15175933070488744052,8557412162374306745, 1363276520257215091, 1206507072649020906])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1439564256118195292,11404518156846947668, 5178720706420544533, 1208701127683364254]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16144253490566453732,4504249177135800213, 367723188584878275, 2190328921017053644])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10025827719085050808,15989316882772203996, 15449519052851461310, 1313861631468371700]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_5() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5823280062939564169,15680090768088578823, 15930926657659411240, 447669662857831647])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7123031478800885477,11854536813934295290, 4576838324085926162, 393867877332217377]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([369312874939631957,5577468896030353349, 16411512831196144769, 2225425689593265692])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6638295224199888525,18378089365476925535, 9535882037841911296, 2499032369420772404]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13635119636550970561,6755198425354098277, 14942399890630288505, 1458437123855160184])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7719400999163966459,8978950345857190867, 3976775237719123842, 403317986652656893]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_6() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6323944148176042385,14827035312785847748, 7085342050920843499, 2194884312546864639])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10713727812974110426,15122847604125151928, 7969503129420113999, 3072497528924008776]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8245328154565749483,15008057688241463482, 1904435821669162144, 1441912631713626900])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14067806317296624525,5017805370971101456, 14326143508175705321, 2028047398688701706]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12042115942287690287,16050192987958347428, 1145228044111305845, 2627988669539177495])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13047123149285011562,15386613089455414049, 7510911058351255393, 3316332504285088137]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_7() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7891890699752675087,12715027985949561209, 6042813899840893100, 868694344373622319])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16399427437888050820,3846046456776588454, 10686023346950737987, 3344759956442768000]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12294815732816214463,3961530758447984123, 15628978538598733560, 3202928354188042095])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18252201952929349815,6216908994703533045, 14438125476595334964, 1047171371043863825]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13026834433791157246,14765348982607191910, 16505480142318392620, 1882850159514956635])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([333599781101283695,17810853688536321163, 16496026278973326277, 2847222041893320289]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_8() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7116649517358614829,2920647569665513542, 13209731436924262946, 453639253430949154])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3666803213437464831,7451829943988360517, 7980987991301795264, 2666344424132976136]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5219278092499055320,3884916115576163386, 16328782425056420861, 3301475912234288630])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3431571446564174036,11493009609057756909, 8618636858343857939, 1986863209228296802]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9074211749247733676,2418817438739424044, 679117380560856971, 169995907362283696])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10385737847901021227,14315582513327762173, 9738557493926035032, 950471749667491902]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_9() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5252025707559437409,13953637720164835353, 3419520033516939334, 120015468621981738])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11981700070783507295,1570822996567796465, 7857295077868291515, 2144628409971280383]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12620013865667236324,15851160004860545728, 15601915586305788059, 2284009028467699413])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12661019300699867038,3364324049985048219, 17977672049062988437, 1576103176157591547]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18429994302768323336,13813699275930670289, 11011949919505124260, 2042408909137613241])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16574176000712322217,10294407224739185804, 11693088177603906544, 639379204620249221]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_10() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11961844684395492461,15529371282921715378, 17772386125370065890, 959811724919934129])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1378846033346520785,7389322072149155598, 1592202723981715011, 2731071939212094142]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3639945603867169008,12646131179701256118, 5410141380190428528, 3332492611662908087])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([270412619520218290,10788974735074041043, 11640321094267608914, 2983084359777738873]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12862302995183110164,11021245783930228767, 8237907725727474589, 2863977377025820330])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3599080925301125251,14847468876278789756, 10377513966858151960, 25009495063667099]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_11() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7789585186810506137,3142937920393237003, 1333992290293988007, 3405117709433363837])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13941632647345873213,6335057726077175542, 4393742599438708031, 1166373703845352173]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6390806688290935667,12163093867902728788, 9027613641070652428, 675820989212401750])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9082582285834176161,13942698388996023976, 12899799278085120109, 172651649945782945]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8118044473652503376,3972860392159575080, 18001970451461929197, 2745036824307698284])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4725386799586988925,14574985519548118755, 13381024364756754960, 538854701215780983]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_12() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10199245236306392455,18140621211891476488, 4758321317734916393, 3144595063115386286])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14898452150870613763,9487660811820973703, 13785334839928419928, 351263958619809824]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12097172679406209335,14002882918970300028, 2452894575293828397, 1019803998844768462])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17022629186635173232,14815794137094443759, 59147758509713347, 1324693871018383428]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13003006006967630185,14689187664240821004, 10937745255676672023, 2663930550530023819])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5770922591673404829,8464599943760791237, 7093877099597617034, 2738298317153082672]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_13() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11109750628247559454,8796276775406286878, 11575195341794891410, 2389857922287860645])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13291332001938437743,15968693862995949189, 1331609001848773732, 845105697173110174]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5555725135990097571,8359307485281021078, 17868686353996897266, 2838730614700819494])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4036007270005950917,16917628883665513001, 1930259442491108057, 1741332669860380942]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5586692354362542943,651837647162159424, 8691242988124936691, 1385349058473676796])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2457040721467392298,10123092148601827012, 5912322787897649567, 1752588162218680612]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_14() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13776818487199148544,16428840051404012162, 14603768868797311081, 39026270245661099])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7371453859000933441,2748883790921104569, 16310673174402031236, 1649547947142404824]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16121307008453781352,8906102328050185959, 17170323266801431782, 1788962652510781744])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16877358414327449982,18399747750372623931, 9773243898135209257, 725551178318164664]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14713002840501485040,8246605639474105338, 229732633260237634, 2530982615656977995])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17357685569203809992,10404152319868930450, 2604386299360141600, 2334543948983597560]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_15() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([270623735424609756,14918163881616265516, 14901762973229713371, 2337283132865193098])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6292997584620635612,2144833437084895902, 2134378183304920992, 3349347073273767152]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17676547966088076883,458753872429696841, 11761259982572545257, 3267428759105239811])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2601818228172458045,15632739716319475472, 7593190040129529217, 2528421024446889529]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([52081136084281038,17559799843074941902, 15222581854534887623, 3101497702803168821])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8837388507886040524,7044691055583544131, 2985446620859937354, 1889381831823009255]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_16() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9331137865173431579,5914118189199684899, 5777934102313478341, 1570348050393075155])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17058892582591333113,16526401512954505844, 6833232030632977913, 1570516692309383007]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7342966309815578438,7537828808518296880, 5113539973391917665, 810797381146315792])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3323792432157577081,2371595650129315298, 16221184478425155365, 599359364534113446]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2335360793943989129,5186525376473818449, 675441685282991805, 1469820100822170111])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8447861390500647265,13887404727194987414, 7208527949506320237, 2218806390448196634]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_17() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8504167964167229421,1987673739983219397, 15695548489679160881, 1269388259138855172])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5699164409608181082,16943870829256571249, 642982170715230164, 2338598723983685377]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8041715062708232481,6181628535962690722, 16687549753794234249, 2408128973977932429])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2642837507779895735,9929455706859507264, 16516509058985299986, 3137964488796391471]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12562973452466521228,3911010946243569277, 16590246008846385690, 3416585279284380113])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11979507484866195887,17305143414537139887, 8613122189781079270, 2945902600826310694]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_18() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9333838327472285682,9617868530653233744, 4816427301812528335, 2048661490922076411])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17618791627439204140,10703376339490122766, 10683950770439504307, 1218489958915303876]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9243463686563249205,65292291085296869, 7672861835806619808, 1094869345595923609])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10948739580193828120,7308298784673390599, 11981784362067666487, 3240665762926479152]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6203610873777967397,2738951366607438922, 15888588011766250519, 2916760111404819866])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8262436935845292140,9312906657530830944, 6342771037362748205, 1659944025525149408]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_19() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18215517424930349890,5840556657713931368, 9726407883960506993, 2733222446438796815])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18417881114284240262,2482139030217700749, 12651144484493938070, 1778102854459179110]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1917567522355933632,3472920225926259847, 11627235905984107810, 1299194979255028525])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2734437922946594316,10025277329724169898, 5235261717006898493, 1327260968417511176]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10250183287278618915,4151673337958646318, 17710617657432679854, 293291523528482704])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18320702472934346261,16514596468241182434, 16851076470380498635, 124666343977784341]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_20() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12524438342549689577,5459481667655562933, 2221836122553761812, 2705195649187196137])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15909703146204720789,12131145516101663775, 8211724729294568100, 1505061990392751464]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3226095913684730576,6186281304628777765, 14402556762457030217, 358278747336173175])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12039415121966808908,5900712388241029694, 1602599201193736784, 1815031710939884417]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9512238475105550544,5874643446923927502, 12215691537564421078, 3056989570116418167])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10175333386773080829,11724856316761978061, 3360793854081019486, 2320196409143829544]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_21() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3448963930128990034,12901635784669426942, 7828660345994972202, 2116947423036271518])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9414229755864789752,17757293741410830478, 4382661608433947301, 238452403777729495]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6037166602713789517,16155624344065464919, 3981335822407313408, 846547834482100606])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6703252619463926074,11776578003599003015, 9239601730106065788, 2065465790354154486]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5126522199413229057,9866650836818007137, 9913248993236841148, 1875131280615191969])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17103899062020561889,16231513481229464812, 12555789897688984844, 2304275733230114852]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_22() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([287109964313747249,11413262288885818636, 2780078482931684748, 390486440201454578])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18441510591766573402,7220632814781873578, 9367905646664557674, 50888335744684078]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5123043798498298775,14258897169578363292, 8639034688359796122, 1307358757862109190])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9949337711694557619,4190678447805837035, 3030679323270283036, 246924042674169612]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9612367799524911901,13944124324546598759, 3933530899587146181, 3447805002723971132])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8480116776824046264,12370791869108421604, 9243542121437243874, 562931378724632129]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_23() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6819347212443508627,14380817956009020122, 11490613643786868874, 2675884994885701690])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8539140479632408989,8194211470684079269, 2077135028506898385, 524589038608530199]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7238320634589634943,2810869298491860188, 18190823316940059167, 601328476248575923])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17330135560310379025,13326842942671423959, 14192570876734829607, 3187189158523402450]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7347441058293950809,2381654992628648694, 12283127418585427050, 554127190895867299])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14525920750940424436,6251376237155514255, 9119745497840928963, 3026137110213085883]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_24() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2415338884681002846,2912680339196872922, 11710442952716234915, 1744538961600730111])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([713726665293542916,16116446517150523815, 7301716985417093672, 646541849301241213]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2802651880837342007,15264407571275126130, 11482219452212366791, 2580029176521980262])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8051384972651499709,13509097628089186570, 15987139753249728088, 1724821564430626417]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8664415540163095049,12168471377940660156, 10660581930556816542, 2985417154117979585])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7215323165745287777,17512809659064767494, 12967761364295926488, 1156086464150001555]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_25() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5847020509332917002,16107840255528183857, 13424777394943063602, 1118213906854175275])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7302759526184944596,13438215651397792756, 17503339316445158656, 2803489848961804495]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16118693761543242214,539489330238339253, 8780338381895319476, 3091216389784702423])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3819222424053953391,9484305177404131071, 4427467557402947400, 1081013420385521109]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3197682013228612250,14100609499874712157, 13041169619542001726, 3150877883620438330])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15031901740242069817,11552635673714776172, 13108773350990590777, 1584779323744170842]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_26() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7736666091845641957,18358818191766746013, 1679233114620004144, 375335990723118298])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17733721025036990864,6393003824585435020, 13936297069596663203, 1893327663359801799]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18093693628223309491,17936637473177723327, 8003761580206033133, 2282159549033336541])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13237504704449068028,4438394540129058755, 1419876011796052562, 322554199750204231]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2749757163456989388,3040049802220625708, 12838925560779984220, 3418584958513450119])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14131965388775222832,10546007623566370784, 4986128003727216251, 1822858366434246446]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_27() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16609168794555169316,16493659075023120097, 11085579940439096654, 3058228494886721962])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6282040349389757097,14665126689889831816, 47941081847894345, 1159320964236639124]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10106442699566474898,15481463452236569372, 2216549774949871936, 3214875185114687074])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12559738513751922886,11635511430638918878, 5707967392493025109, 2697916725532091242]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14166363440832640524,2696326784354862706, 14232084503259944463, 1343655363295586643])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3968375579488864249,5891246078049120946, 5881896279230690117, 2297090914499491897]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_28() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12796089133672114267,13701325530126098697, 8552244792200768737, 251608614261995397])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1739651274117659838,10383924183274804335, 1232401508593539744, 3192787264745467455]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16131052045340976757,7521662895573958583, 4066729838210315437, 3246864608797333303])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3556682161449420215,4447305674296501708, 16756527478588630418, 1611660486408356589]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17007742788691177755,8528942540112163609, 12106460327544228034, 1689339426891782604])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17788581739690298860,18079010206976808141, 11311196041425439257, 3128397073005990618]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_29() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7399514695810299876,15915840024206767881, 7861964664907148309, 3128730485594524316])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([446723938111159206,1287246602372247874, 4703622397805887749, 2094720261554967625]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14360350713707645204,16896621100042785798, 12877008806139365476, 856443816251001523])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12227319666002125525,16680200133876766018, 4570629093810756863, 128061889664448605]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7638453536790487049,13445264415522465708, 8959408682309985432, 3425125355783343221])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4941642995010997310,6415545896376924726, 6286639352509907220, 1166330931770153584]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_30() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4138536755720428249,16213310326368166523, 14764021647284744860, 2364884853001270249])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18155959633571591871,16139433975535113501, 435080588100196060, 910213465518351194]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7662805989174451489,13192898642512506288, 9837691826938349091, 996548280215820609])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14641212265627999576,9971000290503377956, 18325498896227450631, 49306008440482674]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14987163974283789123,15369367139258711381, 4805108040536182164, 865389779847616075])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12780902668628266944,4670264411791925444, 914787537407591974, 2802652746619412481]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_31() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1543219374122905771,8090450085994893608, 8765342238743279456, 2645906120949195560])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10590034945958063776,2148406424358347708, 11445056787421280387, 210343342343922388]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([84160011354238373,13765682145630609955, 7800796773103963386, 1406300471758320952])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([710490933452581864,18193564024607700386, 3400943045746472605, 9575821229388628]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17760125995096386084,11664557531912333499, 13522375229170762481, 375777927578750945])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12565086070997357285,9691086342514185574, 11509833649633872030, 1610814813808774869]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_32() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7291542090865801949,1577101696128872745, 17460728556119018831, 1481908667538396308])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11357486338903428945,9402931630429116071, 931417004746301649, 585421111836374602]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18076471528705267126,5349588559294005231, 9256503960246701550, 1811180944784968872])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8344292930152609609,425056255882468959, 13425843550653486842, 264791131786334434]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11086156574821929778,5292553283741095858, 12089065119319887245, 2319694087620793161])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4633273787664257371,7547310177033538971, 6601454254372768242, 2712998898483652673]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_33() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15409796379905470930,7552359155329271944, 8707799418017204781, 2077799144247017100])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1542392928164153258,10603876708432473252, 10302427976007626990, 1603620034839836595]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14244300257937697759,1487105642315742749, 9475997014904662615, 904186338103397177])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11713173046184750089,9160977860234461357, 6078171397054457180, 2002854249002367041]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4780673991028727399,4569514823412893415, 16683474281454486792, 168860920449206376])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4445516531299081531,4336300495570212604, 15119934909769648988, 2113658241731487389]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_34() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3739053348046568870,13759850316338548409, 13717292334514285466, 229901727331435613])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9837388131089352616,13837864382286695963, 16647339903286215086, 1392188083563275559]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5614503646084565013,11539317751278873413, 15777690545326792255, 1179853794666144299])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15640529013230252674,17167675400528452069, 6341480850007228862, 2516290777480162977]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2568985754653549489,8331258617886507398, 15289896141672320485, 3111748989238118641])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8520725304405802997,15638238802437077320, 17665258704505957543, 2731156151271490823]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_35() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7120317261476557715,7552257997228182021, 16358236190799863642, 1629867967602944431])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2926655896310185389,11342398253884210994, 14243889332822464811, 5673933864441291]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15263597988564605707,1834204502393602270, 2781236779387804203, 335388004383960306])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13837667168429014907,4675266620119564585, 17991599336536916282, 205128786167620998]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12226457088307444571,13281439801408605565, 6871025939324737887, 1355231828771092400])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5848111637689269596,18344987805008892157, 10290344629027789717, 2394074559139099143]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_36() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13555239015132717788,14924527318658672652, 9612862511435704261, 1788235917197541149])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13743185498493428374,13042375823225200846, 7761942357328267608, 862687485163677080]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14465100373986763913,11398519268715632530, 23522222063967972, 1057670260062133535])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15976699050801485602,6319788719106771266, 5566995811024330042, 2387504266708092562]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16569113230526403569,17725514535345950488, 11866006831990305753, 2130996117207853294])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1527532136197539549,2786640387952763279, 4094770523233825381, 1206940085346694695]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_37() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5114804075405120203,4172763077557640727, 3328951903873692349, 472711820379981624])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12486348542366902455,5733542869038903260, 9171736716159526893, 347286838645629371]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7426745889640868715,7336034198735935872, 225818981130895468, 2851352892440829973])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13558352020304174794,8731800623672578152, 8611997025371520380, 803901775735360605]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4306725704066455433,18139295094576557768, 10045251235784233754, 2086969227141113700])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14119044221441020420,6574752427733012334, 17898012107281779637, 1994133102098711084]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_38() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7155714555768312444,10481259648759533641, 3320847119085430469, 735361489356423879])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17450582325843748451,9873542994348163314, 14772659291204377472, 626502329742690870]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3371886791869284567,15381838635685684107, 5657371297269882352, 2651932389293531103])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12269515269623735501,8918742107553422877, 4172805128451843932, 2733666274434653456]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9573118767154288402,15470856646598692794, 8105024619525140398, 2224539222662921593])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17034452844578716199,8554107061262379270, 16591785574730849418, 1633226586394386861]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_39() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4711161931912789291,18130371323456201232, 15277646067184296809, 1342727550878387191])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1467126513716047660,8021413539023584799, 17801508658762354334, 3213122867712408619]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5853050774925202553,9705908432060068641, 7538740777643461165, 1804229708703189317])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13532391595908283977,2706364488996114206, 13664153023687637490, 2790608905367055510]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14519164874451284801,9104102277442927166, 13279729600879296984, 2861474850255824756])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5377949737843903762,12797117373748448714, 14215446886332233979, 2017226962682556292]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_40() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6267645080032865320,10750275706111093063, 2107525863779491991, 1968861127619145353])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4415519846160173950,11096921346300086957, 1170710085091004866, 72616782227125758]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10232906705228961780,15876358800657746107, 7449296634528376275, 3191557894907054641])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6084185408544380217,12166250516002141475, 2699655953210779176, 135679131148756839]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13839678956621451371,5837800799471155642, 13017709583009916558, 3397622215689528221])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15500223681764224151,10301813571936861125, 14855805854578839215, 1474196592120957640]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_41() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6159919441405191294,4477945359632189959, 15198662699206172876, 1280396025645841436])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10477543762425302075,7064537348438568871, 11743696811261499485, 2383919542106740699]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([248751161004592530,12216330140153424401, 8543578797685815603, 2859027025838042733])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6194656446907457552,17876752927769885539, 8407565900741576915, 482489911112910917]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2765500164015116551,3029999966476280962, 11243794975661292962, 860337896095117828])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16994853261164388493,10277461362946878297, 11170518452121706239, 1160792507953008658]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_42() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15529561912129630379,2206068326658408061, 10937502935126907161, 1810329932763296750])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11773988251929245535,8168905259549524408, 15881146625677964247, 3301148154720107604]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8727185441572338877,456204486442982053, 8562669116275066184, 1105708298348662747])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9870364555866928692,232546030253946073, 10620863432028529411, 1618420585265274997]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10112613435221297756,5169148670295997689, 3957217890151964010, 2979695178810704664])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8131745237183518180,8781822738273576162, 754287471595691592, 1808012455343298918]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_43() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7685420508597090459,2244927223990886702, 1415534581430209074, 1799099260139195975])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([37517095230738605,1692194596799775857, 4156007241714134993, 1625175245680050152]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16597362184989623665,8874879443557824057, 4575284840779117537, 818887851400572696])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12080683947072644023,6758163829153298377, 784679329732419071, 309203053972157318]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3952837571701453180,1903135870921686260, 10457117118987937084, 3405423714905158163])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15017229640277038985,13803203662891018815, 8747510305522008880, 1448200982938534942]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_44() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([556304748214622935,3135283039681552363, 16865373005037478071, 2938350916681646433])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13050139955487689154,13333430870083485265, 4329060736223953731, 1047348912653006317]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8770383764551582800,56366096034035302, 2685154886001377122, 3249409559619901998])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12884160568804891593,11581608250361043986, 14196902620164457538, 2578710095761385912]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15546851523730721805,14092579518078533720, 8871348309890717741, 294049289375972753])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([342085843141481611,10154361958468512950, 11174549408220543679, 2893600745735005009]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_45() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14647082872060818788,14817078032273354816, 10064634629631738490, 1606188032088004980])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3705444889065882940,7361127136866573738, 6520054309572279251, 1051268700445219106]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18336063286515740324,15224883798088497869, 6278436591878330149, 2163055900135459124])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6793070038140550276,18017444111817406813, 6767301765502429892, 212697772470584297]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14535225152069889826,533382451973379504, 6614622700216502268, 537210752675714696])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([214839826762703769,4686888271532457721, 13579582382283685089, 2716294784014812582]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_46() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8355857482047942794,9648467663681812189, 480005125199695496, 1250818138525467346])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17343404999127466843,6425518906437539942, 870875280672844244, 3161873369949566100]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17907793707812022497,10691554820921122451, 1280459195158690356, 2990099148364492831])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3617438327511701730,12341708758349691462, 16098706603763747897, 1436467705391832432]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6862210410128284842,2985523853272151856, 10472690361103266932, 400516676924338298])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7221846984979338584,334208652697608704, 2438082793402141192, 1673680049462395272]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_47() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8428076325473342447,9130298170338116706, 14251703806791933252, 452784505910647687])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12548639873433632870,6647155020402947175, 15808303000429910320, 922278877994736356]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16670237437411228134,13110053254265281925, 11070339283954752612, 3344003398560746676])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15344203836219615739,16450005542301114345, 7928336079533256047, 3092930395156423900]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4842364196863981737,16835448445746375808, 17822749353942278677, 1766986132760433167])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12903021811818100894,17392054340424039541, 2970692416086698943, 1092678685318618832]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_48() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6053571397947271628,8986382231993405489, 4470814686790468009, 1670109159964864270])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11670954415637520480,8676064407044588983, 3493307303700138283, 381991932643337351]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14919872639259527266,11204446196210274807, 3938746915973795236, 193071420051602668])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2169640326393674147,2976414262502596941, 1068741284613745466, 3241248254328953421]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3347050930566263179,15227623261940056373, 5064449734588130975, 1644841272216980058])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1625280631411413880,1702262130639902885, 9529161201018776810, 1829165212640860034]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_49() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9946192486741291674,16009888218582641868, 5258189543169640418, 2800620383260601224])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6260910462313486021,18019731693449785283, 6581400544059660888, 344253000900514168]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10913968561495584788,17286209953281281921, 3832380249816653571, 2296504327540868255])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5980313309144068105,9676642392212827000, 12750006247630515173, 2419885455009742218]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10736283731559531821,469413005744342390, 6693792377875160787, 709913673382984383])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14260366320606871896,16001207483173510002, 9418928439988216436, 1940515962799049672]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_50() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([668612708904202850,3167321822604705979, 1579530902783930144, 2346397967698798449])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14290763155749462758,2407447641980493596, 15407234844910327919, 3384970810715598847]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1177380238022283512,8961579017544796719, 8591946660150039578, 2492736147752479232])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1066371122727170893,10595411439863435174, 13669654662988561356, 1536218391537906349]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3990287772318266996,4126143702790782387, 12391293695136844386, 2226590308183300015])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11474196062101036939,6512232067889266870, 15299488138452155551, 1359612874814541172]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_51() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7833810580934822787,2278173342061405224, 16625781782762714556, 449975474420201749])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15908676400454436457,7659075249519984796, 6092125453849823626, 2347403061601242932]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([704334472188542423,7265255395597411589, 17778200845973254481, 2916194317447545558])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12120008460566880578,7050874281741056724, 16913640816487850711, 69787359441453702]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12421985291619175106,13616740946265748115, 15690972551032352715, 2976784100392826795])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13053194126554283537,2221797547088586530, 5087119149953290972, 2606351311925567415]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_52() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14910168874986442728,8031767862594161641, 13082123848268012237, 327033253264007364])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16437551873538962056,9555318888998921550, 10441499694453588872, 1216857201682350755]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8929484257290050721,12216641768225203374, 17797023415867847222, 457776622682966994])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1003814703751094379,8402320400688343081, 14095669723564397446, 2701594127802062962]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1605828574345606971,2282854075343552347, 5322406924554484809, 1772824224631335692])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7578546237214991511,14700451298734272718, 11949868685548483703, 612554363523646164]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_53() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16667485799333773504,3221086101404825275, 18335055350165017628, 3220724970874634648])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9965961477366606014,9514984276782648940, 6659055027309226244, 432815556239122909]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8871475679956329666,13656411655313599985, 10278285147808794900, 2104758402770576031])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1881275745878281923,11114772088861627375, 10479992110691850233, 463976086458283984]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7107780817631756592,10796550827481526984, 17689566673997668915, 1425588873985650284])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13455981903192993053,2524251370558876649, 8142796561702863700, 1489982824885883810]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_54() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4608410307150166716,9949176550436589210, 8414886195581298390, 3351545680158952230])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12614760649397657074,10333447317345225951, 15068923264347419408, 944617349099394366]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7315350430268972497,3548934616792043495, 9263354149466389875, 1806153137917315461])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([890712979474667191,10947174018244634898, 12411878852338739934, 2406696367944098355]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17753449963041275052,289830216119421164, 10823988019836381479, 1864433638483422165])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12659154223165019981,14291208385812288033, 4211077010581610829, 405180633523493395]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_55() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([512316247260094648,6786053729138575658, 10061032147386610763, 1327278236519464884])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7775870594655883823,8686737039564117681, 5138930906871917634, 2225217506801160330]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17678949499006414264,3046943661362056954, 603782209793683918, 2949855076993879063])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15648404799849869824,8852492324982358106, 7236352333723893911, 2968792539319848949]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1341824392415388245,14839234163860451146, 10927154602245338474, 1723556217420853083])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3698733681168442673,12520078737395306409, 6552099471998238285, 3297070465959638818]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_56() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9365894875577831369,5088926746804912808, 12206089286387205189, 2720900606756380239])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7869130394231972803,87614893153743506, 7398100225563446544, 425580236367558222]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9623386052275909434,14136039815193396738, 1039748712468975291, 2703775407007442287])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9874018372711419703,17610825206743430688, 2030382861131513741, 1089540355975414475]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18198233620107185292,8383715319697940874, 16092961870783587833, 3059404893348632448])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8030201406162679647,14390016679756577003, 281274593549247080, 3355147701823212150]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_57() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5422680240265832996,1946318342117916306, 3664916472817012405, 3241874286956908535])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18311043380692671418,8100242697301145939, 12667924700306939342, 1916567917399583004]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17950967283822921384,16844242693178970687, 8067610294475730584, 2973722185713614929])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1586896194752532496,14530098787235041139, 14057579702446203562, 156923356255504831]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16355350933727732631,11776541475409484475, 14261559773007539506, 2751337877015607329])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([828737256128366603,13583580367718593264, 11493988924326139353, 457769640694574808]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_58() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13003328686856013800,1711598861164327991, 6468057120367482348, 2730935446791051989])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7935427476618081549,9348338193126529088, 4081002799623368688, 928439523363955439]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13002181057977534922,2628453963212504637, 18352375669292150337, 1235495029864377313])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15338249627883426984,18007053840077985809, 12604282715825014794, 2692787787259905194]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9093686028111629152,7994041345650666007, 11608789992358999520, 3223982669655634038])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2992640776169583751,1871142284957443263, 7623327292315785978, 1387205035875433685]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_59() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5463865735891083899,14630160823257816750, 1155950244367731559, 3099083154394233077])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11224215036972939113,9261469020329846904, 6881454910289088652, 671621440445142178]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4421356026529641239,9446130041908699392, 1922014958537615400, 3274503252285438062])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8580277962475378750,2554147701309430660, 11589922152054583084, 1644363429981893214]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2759293219903128844,13723779788575808075, 5429734218610420872, 284270640615813289])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10127298648082168099,9775834864860166971, 16034285677595279060, 1969785921159627744]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_60() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([600580334032652804,2321757057342758172, 12927044650911696839, 912481606074830813])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1742222540423170356,5388411484089577888, 15669036111913009800, 2290353886734264530]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13524932709418747676,8729729059288064300, 13248815784412083209, 985601685628940268])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8095896670723043672,9479714206220833451, 9935357687700323105, 1987935092865768071]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12351501345222280255,4564669834210920077, 13079238967595314121, 2893588379208385448])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6790684858368175968,12342307814734069340, 17437338779261382151, 3113323595042518587]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_61() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4335458208259077364,3070146124880902986, 12132813483765417998, 2189079783051835915])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1273417055656037261,15242501717304447765, 7470293015109889396, 60979368330265425]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([495352304476389410,2522603611861727223, 17400605418483600773, 2405243173227662275])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4145492522477043770,3930196348863807874, 14033363453362515818, 1683218100918056121]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17948501231187937250,16236352245361706135, 13876518441607112130, 1989605743992033509])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16575051902264652112,3638918838457874372, 12639744488953793639, 2286623855733929397]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_62() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10038236537227454322,3834487206512150896, 4837315834914812659, 304845306814793610])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14889474274324897851,4412684911204420504, 12052436362387123584, 2319462749387062806]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6844925402023018696,2984767693307730228, 8452064398111784330, 2380067291807988977])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5724892535103664571,11711437062631280273, 16270950815446121735, 2789330935070267190]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9755397992000155692,6036355593694428132, 4635001509125872439, 2886159028675813399])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16069879372809196519,2291312007277523126, 6433180933700077439, 1486270900233595497]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_63() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13921265890829940939,7636330375160570474, 4665960856949324823, 2637580595337034782])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1676311535076855475,14388116664785243967, 6999886116491182694, 1225600792830782493]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18421970317862919207,1327288065051737192, 7012137319813403714, 2211320130364568615])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17479704104393317052,7470349539550992410, 7224478043021223785, 2243932882698239654]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3112539271062938499,17833733510041471540, 15283182265851972375, 44533906695271561])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17205450681903040044,3522788048615418771, 16922755076551489721, 1439467848016457250]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_64() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13377146695259432802,14100753620496409992, 7674379385896553691, 2573832464465008930])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([557324928434175368,16981228059930419063, 6030243056256668828, 3417519645769543382]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16569293480025381657,10061462583010325847, 1668892605250494803, 3372783123198145055])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2313789391118724807,8732181255536968160, 4654984413883914588, 2797945300820465029]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10786873503828530253,12816469824825706583, 12612291718157215005, 765198871954081674])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16567936393605057352,14195469192344509149, 2677336580189740007, 1591821379742287238]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_65() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13626726796417775597,905039304520630014, 14385262270270118529, 931464122089285366])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6459046813597811608,686563215136329104, 17840121544795996864, 2526159168346761237]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1740725520778714189,3970588031411753937, 15138818668422454728, 3154065812649955101])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16142871230366316420,382147931728803859, 1578398685310446495, 2892321005796480272]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4573347424348238098,13930425395157667377, 7547321432605579256, 2713293223971936786])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2939755484115294576,6869514172505015788, 7002476311999383824, 207732096327248456]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_66() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13709442686358577124,15832464831351649571, 12076039113892243546, 3455485297433323062])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18023010755270710631,1345091752456542941, 7706395518894446797, 3294337764106111222]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17912043887633553435,3081559084720998117, 18054355427023015612, 3134494175808145082])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([186337887500567930,7222531963656893566, 14155938318246282343, 2771351750031622878]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12609246564220273487,1594397207756565949, 9825723030124067254, 2724717191726459635])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12983994115142587269,976540368811885047, 18335483755257327302, 2167427977713783633]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_67() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16767682901827683841,13701264828479258978, 10181214404393679883, 1395082227297335398])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9978352420098653778,15908682512986432460, 13661981653086040843, 424470769782416566]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4767469812439913295,12418131412191642916, 4756013449213123766, 1564404981129552671])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8809079650558686821,2123224115614886271, 6313393965193212986, 3378636516059800730]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17072499987405276473,10034793520409945765, 15759239356590162154, 413675290036181673])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10403544644649022418,4260743975056099883, 13757505939064072804, 595280312120297909]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_68() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11927676530117983136,13988462498903978459, 16181886719869833670, 160879608735390606])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15764181760919483149,4749684855409639810, 3265710675173907460, 1373672660229324824]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4702490859355290177,10088008658301722056, 15951539533040514730, 3112554746247782292])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7717613784005755383,9824452686697220968, 4108379304217917329, 2144271069510608047]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5210921203500069385,8541209335657447558, 1626519623793301518, 2000565976104463586])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14157975516393092985,12099147446097414594, 6648131713760764113, 969754343192134911]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_69() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17980189460187830778,1889219941237090783, 13909195107060244038, 1685396046443038178])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3051833930183928347,18416720040833329799, 10087408079259648135, 1576906038236231369]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15466516961608624603,16844094941723239288, 11285642211676663926, 2975145632133377167])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13471066094520601513,17016282780077945631, 3938930665007322204, 708245874277978584]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15031906007597449965,6239039503303329396, 13836318553878840955, 2695871689734768577])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6793852692303112306,6313229706798686289, 1376699463481876760, 2357506978952490562]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_70() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11366828076657158452,9549996211954041848, 3650044825479732830, 1373814187006603594])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9692203809937244817,7673331868786818439, 541285382427134688, 2059056691169337877]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2839986922434318894,6239121690087443138, 3236924118009391825, 2074927829763140595])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3579273526184828578,4647357847463812081, 10661538303636942038, 3413451426506981126]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14254589568286730706,13380399875864742632, 10230702462255046996, 2216587113744176350])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12638907948086058139,10816299396480670272, 6983807940519337989, 3230703026124390459]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_71() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5058092415032108019,8462765089004818857, 7967880597174752471, 1490019821615785378])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14070960174070275357,11842197375827090414, 1306527162536347535, 1957376770316997198]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13198164446365042680,11901445055653796909, 2281480632337590920, 3142146612845804764])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4156556221081109790,9039985719708943681, 7223578382291564494, 570548833514427866]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14506878757868128098,16544943007052494589, 2198031624981621405, 2033961111379319849])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1569457014480111647,8958345506216933074, 3065879269802886706, 1556744670853561648]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_72() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8670769452050281486,2603157643224096297, 5170940931522518091, 2169635355341440494])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17858655563487727388,13974350819178327018, 14191523303038937084, 1933479454295296944]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11561363935362204025,5244667953953900991, 17590897577082643744, 452292303762752503])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10548293481047491397,3726803642967868876, 16004578249770811828, 1197762234507486424]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4825160836412796851,15796880338500075246, 18104269758408963704, 3351107969528331429])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3677353594604656777,13243886080375460536, 493482802944247320, 2887949280159124944]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_73() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8479760529281690674,7994751242138411134, 15345767225237783154, 3430220711344759297])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([524416989865606757,904006822949417225, 2796530985342658600, 479884119105635171]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([830443950718677975,1025634055668035284, 18172819390113448366, 131764769808948612])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4222005762950425160,17555140366314449330, 14403806003810144495, 2669422862431671005]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14565552002431377048,16565630375138188061, 14575995455443639551, 1274682148956826002])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4889098274062523040,3593520692859002798, 5392306450307316027, 3182134861099261142]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_74() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1309357342976202953,4214876501554558651, 612196342916926809, 2975961936085837136])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10269679222705203257,11780564713707960686, 1794612721178241253, 1534343832212062965]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15678624791121454939,10355914611054688241, 7599887174411926085, 2863204088224670613])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6002955251576793929,6234348676434665827, 14978427608665948614, 2268876856796670138]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9257438800131341756,3130394428690402783, 18230611937698292340, 2740678391300670167])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14390331293882510601,16419122494332313410, 9922013992643156140, 2844396889136699083]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_75() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10098955407197731028,12656715286844168905, 15535148986134773930, 2947605084333243272])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10657330263306238797,2217315742359539172, 18265949491644139088, 846602167354454739]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9038037631799291094,5225532145101885408, 8281059509122928437, 28324414295733782])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17467972194800785766,5536291185643948069, 12736906291079283085, 1852657603193167013]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1516587024314539810,11982255204610613517, 1155001714131273471, 896401992504972885])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17030494227113718239,13630795687692713630, 7460998560730100552, 132793221639939795]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_76() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10176615368683446660,601583534723294556, 13110665065271052236, 2861720736213065106])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17880345649508354921,3871979978980853183, 6655522217469181008, 200518909375314463]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1798745044690121375,10759244287798089531, 8948551391234015540, 928153838998254791])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4854644120111971641,11604223912110353503, 18403836121615736198, 2037443035641934904]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16565873182373372440,7400749194843479231, 3008657753327233814, 839805860278934523])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9378001881854219365,8314728862485991704, 7763929823464409236, 1843342585299060254]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_77() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4856522731738463993,12034688578446946381, 193602104566264668, 2354117359251320137])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13090388147234528083,6770974428891810482, 1846596488981736037, 1971220940796455096]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14016549827697015028,9608312358581675044, 4938637631394377688, 2612555136020187704])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3767156235301953140,3708001549184614692, 15817524667752243240, 2711889907619030045]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1325748209183447016,14610883328180414904, 1759057113030637034, 525339018639169178])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16428758989474667697,13196397865265683154, 9151441209947915125, 3283208787367466898]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_78() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6553874684694368455,9301571099664561278, 14237282328599277535, 2471261460831592801])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1636100556647778311,3827957952009907610, 15551048411498001302, 1898011813581247742]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16950077254605975014,14754718289583278266, 11624264081282056163, 1305554521772707728])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12772965760979088209,394669632841202750, 16394036464624281841, 3426583584086903179]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3965499529118527969,1864766547587760102, 4921104890318981551, 1038463313152890140])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17072525794739801527,6022531744446010312, 14453564009875723049, 3325175676479929237]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_79() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1090926775047554870,7374864286345996973, 5785741914586266972, 2276686030916015238])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9792707039150846344,4516930697246621247, 4448084479452425457, 1996587931051355115]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17379415007087396391,9575908516321139397, 9530433683536101519, 3122784550798977959])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16525951791268729871,16200773062959914622, 17199317757738025354, 2450494661415658234]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17506600053239517039,6744584825066007595, 6840225580237310845, 1171279626464492107])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4265082730179779326,16653451769088725385, 12829350476060252712, 1346333834376613912]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_80() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7432876191203405911,12664807927281999210, 1595364054487489041, 2701447910710025167])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11940602867216769464,7627319978759996784, 10788959484739266130, 1949685945741268869]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3967213674833130328,9488545194614397643, 10379184825751751890, 2121056850668979323])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15678207676027215262,7992288253016774215, 8771697536939175083, 3445525118491567819]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([135614741216695967,16195561188624744339, 7911415893389956452, 2790743246561021932])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6623361901228602847,5489245354088947357, 4262256575266375647, 162980415131956532]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_81() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3519293655442125928,11780008180724893403, 12228611059897045935, 1697727080360643973])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16170302785219017980,5730153183903731472, 4020348276203094029, 366170179338240947]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15621740143426463190,7449733180026159217, 869406022328016112, 3219751642499375377])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17387351768998469767,6892753790464412759, 11476027800764791449, 3178641352535219284]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14840849188449838914,15625289482808013370, 13932615326385662473, 3016388555336841699])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13737340252094200359,828424022266252818, 10426451617748214473, 574784688313300629]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_82() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16227214634012445639,10180816771680461395, 5748796231136300764, 109165530280654655])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1722301671187689106,2808728941222518632, 11092363639505192063, 2725335253653097289]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11681521770028146767,11672153066658936838, 17998965641137332885, 3151190869304898104])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9099300166845450022,8531745995743027770, 9899135786717951957, 208878410924132320]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4766498456916239682,12673053865872984467, 12345543418898768255, 474193184295504085])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10284844796759031945,5244886598751168886, 15962087395817320263, 2880419048435354140]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_83() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3583795318172233317,11268429957831117231, 8134382223248204832, 770582330385718857])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5445011628784955794,8132405898873957770, 13207376010329008512, 3022248363674238333]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14806908376731975256,5676584781280515677, 2181484093012997946, 47417766018892056])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4534754134813343491,41333861970492782, 8049221930611838086, 1061618236260490291]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7633387908047997279,10470260179857079669, 12862306077389804594, 1146878657420453056])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8307119838564452965,13272869246662139979, 4002361588242659082, 407514155588243603]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_84() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13263646417427634136,17355738782685760364, 9357544541436672632, 2221196746893269601])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16541889920093155845,3879754979833245024, 11772995882777749148, 1494809808540438513]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17276331879514065224,7052008681533180528, 13373166147395170538, 1416611334284347550])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1583651981840925397,16640752858233130827, 10454241491761773452, 3392937361912669238]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1353976248177513358,8562732089841534050, 11765294704957119933, 1810876761327035348])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([656799983739244941,8068547632821329446, 401210878746038035, 617449648840504201]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_85() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15439336233301813122,6708743064136549125, 6956120218660849920, 1426788904589992037])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10482862259574859089,2720795240313759542, 13849853619347396249, 451045416005369770]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16629460883034211643,15841847014574277074, 7307152824076067709, 3220001320002591522])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7364705659418664452,87748415856981029, 884386995931112496, 2978143857741552980]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11607446160836332799,5935320633989379015, 1950226349158249541, 1477507517931399127])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13643865300084526800,4288803503352663118, 2234848628509192220, 2162603572046936857]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_86() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4618868288865848009,18154973364001707526, 3008530448383180123, 320132757662851989])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2679164917805814420,1582666986058664909, 18104076937350696389, 2547159725166493304]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([665923590493782918,15080850902817201290, 15247621324780990287, 1373009601597800244])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10136008735044576801,5925439540950230479, 4926715132399968983, 2379420566225313231]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4561965519654710606,10785850761679807080, 43372938273663600, 3414280957004115003])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9244942981313276843,3041430537908235615, 17139575369328235653, 2256865004348951090]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_87() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12022285821251125120,8394710227083158936, 9190625303714994848, 1316584088401679566])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2648153319109453592,7281114132632607450, 4723070212040088738, 1264676797495982032]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16868530150508351437,12130043265208003121, 13122728819502030419, 2305990981832655472])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16000751638757593015,18294701796841483430, 10785978653820750417, 2876298256075627220]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8640937735723485698,812414643428927659, 313595488712102211, 1576472057062325075])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14748628538018346012,16530758572193484058, 15555401233343268105, 2341080813301348146]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_88() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16880261864759797230,11149320296478205677, 13670673657387330497, 2008564746625684525])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1950835287171218464,16239277735248765879, 16663203488411686767, 2143460148601497489]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17668110190351798747,16869553882511565562, 14545293657630083960, 211851059569339709])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5286719363536782032,10967886215506487151, 10989156048966279274, 788469298650854834]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15938110350409015742,3964858519090176321, 12162238490720959149, 1789796613784663246])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14738972261860833048,2448765326472352742, 18020981669628922767, 2543466490105874023]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_89() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17710901417794188576,18236591538600929218, 12412898114312061470, 398462371967701669])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8909664890651079601,5380521210117490734, 1250490496235015662, 2362333341771676681]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7524201333331065030,5738359738418169909, 6363940170721752921, 2937552274851588943])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8641970889876524681,15112341560553929654, 15325233780144964676, 1970929248826172742]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([985343584010635771,13139852461916839594, 997942083546923325, 1200546018464584570])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12833498188098157810,8667921397909111577, 4027833419537256378, 530474779784853633]))
 		)
 	)
}

pub fn get_gamma_g2_neg_pc_90() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13996630899790034017,1290716614717735497, 8575162876585456788, 987692379801789046])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12847700138400039910,11302335817046679317, 15327917039527460971, 1379901220959135260]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8107823658327067608,849344582515770278, 830549737769622907, 1862251517777338692])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8325223645169189936,5839473372025888201, 2719260694822918577, 2075505971877390177]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5131551208787955579,5038163076084876730, 15415259148435454002, 743545834700814883])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6452707443835080404,16592201127676679593, 1209024538326743467, 3342681632315438568]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_0() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3724428371745925453,1829663926476875085, 7278839501266057900, 57019224021264733])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17153019059173520463,510827560506985350, 10485865449639317, 2139826082945206362]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13953160151939668539,1124891370154222755, 16496622890469169341, 2228899754090054429])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8718908327852908175,10165506531661304254, 4977594090485133983, 2246464849272543067]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12573532728330058408,12673332068472495688, 13225196276178226401, 2205461193378267036])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15273775416471413993,5405125711732513871, 16706014405418165844, 2135349354180254998]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_1() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12146066107567825708,3078477353402469917, 213182695230341523, 1398913300904502910])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17631594091627217213,8052281561504549024, 12482312553758224055, 1997206238322693977]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10363590909161329913,18432754057720361944, 1126823200148290266, 3156978586112953418])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7279241443535064756,6666457450902906878, 4774793240361857194, 1342692613321978154]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10426825055212527855,18036564425018181337, 14054584444173385743, 2017125549703195192])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14047575118215969204,13656168219069984707, 831920971659978749, 1883606148782041158]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_2() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10531339285457157589,5247588504483821344, 14463001314685837827, 2299975927959294230])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4359907786196205151,8492572672548607972, 9792985290945314641, 2513548475923874777]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3795376014266964889,12534190937892720417, 3632106078500004150, 1323576703998641731])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18058485291604578048,6647800055452550507, 6048467336273992957, 1992766104674541280]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2592209160089738801,10369912264842911521, 6776488920484343452, 1765784006649888819])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15242892302600897073,12311287513571279810, 720249208946460353, 493867933958534087]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_3() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3711181289069010074,1320517678748662282, 1029735089875516385, 1401552165452534755])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5407786860269306308,2652661517246442912, 4798861684660580757, 1873849413877086417]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7584389713440699928,5599629525607585649, 9588268915547592387, 3028581625581868793])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4220478060548064998,17548721795362448491, 8743434321631493902, 3288683073761020458]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14940969427897256415,10035048240562872059, 8939933029260298770, 2216302629872800632])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13726274720978065376,9802582714620327794, 11338855638651362336, 537009611937059102]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_4() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14632826007408830830,1329016485184474061, 17366798899150717577, 3330685882661555816])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14528340181317438440,6617624794607655408, 18322293406743429870, 2851279146599319437]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13964668074522768028,3938164411957860601, 4690952642066176467, 3075591251904155481])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4631756664999812394,8850118288460182711, 5935436267528582402, 3271349625596708477]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6668619341873164491,13350001714149684493, 6195933773177716629, 1008055113217895164])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9476821395483533455,15839643882296992124, 8180553240052818085, 1204339071221457041]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_5() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16664327303148096524,777445599949556869, 11939879113172260117, 2095448806442077188])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10941455193589330496,10616702361131573881, 9060920629168038972, 2603983644267791351]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6542468788978512899,2934783261855954405, 11557184024086556792, 1821639049119627407])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6841715599499803789,17234212838617258959, 8482757721010751911, 767242301734257973]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14631234892090280596,3502375450368978531, 3160704388696879988, 3272611352743684920])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15040467139450174171,2493590409239707617, 18404975784245258396, 1322826223692321163]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_6() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17387348522819040249,17730988268609732269, 1991024557366635371, 112642639954554492])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17463842630725249477,15710559528675492655, 337327021142922839, 1801856593710150269]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12542437790837749896,11338530491217317924, 11622396986402423848, 2837278900827682824])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([321024121075905719,2970181438043437305, 12106637101564432766, 302727484288257412]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16945943546026404080,982118997000179622, 10267300597174335448, 309096230224125155])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([199801984098125449,18310169560741638945, 1176492001900482094, 392200076594760268]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_7() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14504547222074579875,7735698468528996942, 3836034103024387637, 1526083396922300974])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17959790229434453864,3626491934484468374, 703571236988651754, 2569098435237806546]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3817460438624308019,15613685038881937084, 15425507801281192685, 3226468700724623950])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9230990362969587022,251216108239588329, 17859000501788068382, 2368466087724203803]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([945088561076717738,4583429267148939978, 18330829762308060581, 1972553090741149027])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12682713995095765765,7032366322323436912, 12858525593356083153, 2785272776223120986]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_8() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2599509958103422473,9472678410020838445, 18417396821375489278, 2579917049268815533])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13691837585947461679,12561305656708031086, 5180131936668139476, 2082164018290258571]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6636105657643572647,1642687756623833867, 6547051125250330234, 1667858138376830362])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2345772443282355580,13172613503770911595, 1954610178282898133, 476326717913126230]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7980439616201674008,13783544295809471783, 5210021269196751287, 1369570821952305184])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7118313646382994030,9089082178049149965, 6662221501775071360, 2164179773388785952]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_9() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15767221780756150886,5217744884508279513, 1584638015522009643, 1384837854619924508])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16954331870110576475,3707372064969199715, 2337712208422842967, 2750156934965060960]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13759689110412515048,14913336838359671450, 12842605240592178961, 3458958964079718347])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5481497710550547825,7875420347659822113, 4345108343109845637, 1780517701988701910]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12852862545205726346,5866681148186487328, 10119445158948191314, 498360798976250565])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14081578155069109252,918111262922635068, 14331536075828368507, 2662308200442920884]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_10() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5551291126152478640,13156702556962919200, 15404917849231023647, 118117240228231642])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([622911958128925449,10401954088902109478, 7731857594549065555, 1979035994551321817]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6094117991509793585,4904913878606292827, 40988880411080531, 1329749118037959483])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16228886850686641833,11243566361734667817, 15881398354492389866, 1972500527900449813]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5225247968147782081,1808932628203334049, 7230529471644810535, 3453095754641860219])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5117884655482049288,15793848981221634072, 7664184751233809769, 605962404466828119]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_11() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4930377583621371231,16598635363846070988, 6190508906238234788, 2236243946178826334])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17527245911931589270,12822455207768828061, 14472165966213255709, 2666601664301576019]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6167633012928214414,3371758434677360273, 7434872450137213799, 1587366058004883440])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7498692720965657392,2174421849447330044, 8651518113688309184, 1754605326373907331]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18289992619670288783,7302620585003926937, 11259924015797857601, 323866923539229896])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6570986641962616180,13074022558050114550, 10137995308223232283, 1561417971553494939]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_12() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10516048664465776062,1788311658415287070, 4134743583158607692, 1792627683456694582])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1271229126345172094,6601366613183363801, 12273879799585724681, 2400838316603534613]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15028689165006360142,4252286835303199883, 6326044162694286251, 595244260576668226])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3183419813991113051,3739735047931845621, 13542345120217478150, 317372724435275860]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3222253367105085260,1444196277198603109, 5571080732297685119, 2435643862273549866])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10245959648073875562,1998076262614669713, 3274043140895258498, 1110471715026842977]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_13() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15933671987032267117,10031379442697301824, 14732654643128877680, 100114074862471821])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4335951239236781840,10830642981033458331, 17243031536788406499, 3345669282579851882]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11713059883948946400,9221618131734560990, 18008502871262088983, 123374923314732093])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11925359809194370717,16301955066624842584, 13571754904586397374, 2103902061876906786]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2053066057426052825,3124919342175725877, 12061106693757410421, 2799627890508755181])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7991311535663948325,6407877125982911207, 16363319304665962280, 2489315485096421188]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_14() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15668926079837968351,1377596410615884299, 2348827961087539994, 3465563991048320314])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15365055623427956654,15612330034236742873, 7949684506793739520, 2349924777017042333]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12988112535993825337,16400789910534857312, 4639265359592901812, 1509389056537421050])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14816874932441237925,951111697292457022, 2577130657856429865, 2520390952161656782]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7258144860550185217,2696527578587628291, 13111106145499946270, 3262090667943795296])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16338416991603466512,1832908815810354808, 2294733937253523645, 504757950072967726]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_15() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15995577072664256322,16403818052657061331, 8377247478427641004, 2005945685370972526])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17421848733166534349,15601253931925683857, 11161847450725380306, 3230327543058322013]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16189868420192645263,13160875727082528617, 8573284750918313290, 2480101140947651098])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14891677286440469647,434692555534628169, 7799397249228285977, 1695273728750520733]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7050128471698334550,18267731021617546912, 11873079398287276201, 2595604215465909108])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6182027658588021339,11331175551059317939, 3413436096061958429, 623746241306658818]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_16() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11458539434797663113,3348646360879836485, 14481927591854936454, 1884360307391224204])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5694224016445422355,6800989728191986431, 10842968396889765406, 2099818054539606129]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10239478937294023547,13822727247829023806, 420753201565889257, 1522009080597226309])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8307278311984801397,17200802685393222678, 14846029035666668333, 1905134696643603849]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5801892660377143974,7737695425444529562, 14188830959581940005, 3304055319590353746])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([675920039180412489,5670160600548509393, 748370959173072803, 2536119996815743742]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_17() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11316363221416761243,2641897812065306019, 11020094787182999315, 277325506870335094])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1240323037706009185,11889070426968470114, 5115053506706177583, 652662514072245568]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9020002490760036969,4920662683730472042, 14088207751516353906, 2178256705568310275])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13945005564221090357,13885686400473779550, 3104465015521461906, 193511830963005855]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16331944008492584547,5444280879780576733, 8004594159114467815, 2028835395039536170])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15895540579602728956,3421271396901437547, 6145845184128062076, 2428995989323796468]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_18() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13742193624621270764,16318539651373693183, 2362800438793304257, 721919377729060365])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1331622337990826742,12809562459351554925, 14269834469123557133, 845205655583471354]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15877365195508649572,9180187186570340299, 11545855834463656513, 617413700418416570])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11229552486948998631,7148445645157207491, 1699804957413014213, 1183025730156863944]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3225279412677475689,7327529370957911150, 11115119106937846688, 132317288156778467])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3889201079907932726,15912157384177564177, 17970397472433548711, 1039569291938492236]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_19() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12802197320934379687,15059873285452898211, 14773481652172528317, 20568610067243590])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11457070424793685184,2662701169096480889, 14239678988818586728, 1948948164348530038]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15149367557084863567,10306308321781489076, 2975120772807277163, 3297724009488384645])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17275837214416998154,10635437303491943980, 6713035479651118216, 833053102467993461]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2334321046275457866,17712886223302320648, 3384369986720634073, 1938993438203654513])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17916752479751265311,15737584335627311739, 12880122008192193308, 1336220064223605880]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_20() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2878671707994530563,17255633040983284512, 5897964369095751805, 969297873890706035])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14129984112742154867,11514543855113223487, 8533895396799075984, 1875433275583484351]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10286554760039108942,15674324474355371371, 13337120219717826737, 1689761476883312019])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10595050450074141863,12173072476363557818, 4487351977160783314, 2750764302970744557]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1394129686547697846,11559019336937249411, 5140873261621148678, 2760682392966666345])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3623988894521223028,13120605539939090125, 2525167513950554875, 1606344848528368447]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_21() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11611057703805499286,9844878333534862566, 12779282458024989054, 322716870901685532])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11712865723547679473,10335358115695877182, 8183415764153476684, 404067034247695127]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9001758843198441919,16428656895183096644, 13875918279107132131, 1379903440994117219])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10245940384898290020,3565344451687193994, 8754279857484255399, 3022961755430889693]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15719061727184647107,849923627034146224, 17244874223778471589, 1716243946306913339])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5199585103573677351,3875063942364329324, 17851166006530114418, 71986744917191671]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_22() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18014838291204575932,13852059177123837160, 17056408688053958325, 422591607943582603])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1384595560357149113,14538644287284094212, 467889648581282287, 1520879249321198779]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([122813970289239021,10249785464502747638, 10326707846295632800, 395599130641938256])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4557645736026428666,1826942302227995763, 6471425458174518967, 3148111051136785535]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9421278685347738005,10465564258377437924, 17379383132298211220, 2332435513058898310])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7687902395589131166,13297436034278665824, 15368262140198886279, 1935311588192688597]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_23() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14974592140840691971,12580914967297190759, 12544326257084075440, 2896508226928209322])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7645027854518929016,12636011351744015108, 5110219921258552978, 1013798250089803090]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7290691296937972432,8706664379607729226, 8642651417782944203, 290486629640182143])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8262056389868237367,16759987470001784487, 17218069247679873961, 967048764163469075]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14235850442627065984,16761668236239668058, 7636354810559931442, 1395115687384145351])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4446817703989728893,7217066278777163056, 9218158353789194869, 12359980780510788]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_24() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10397632016993437067,12083333240375095213, 2668212804199744013, 1718400434787926147])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2027313845124569043,10742620355570408936, 11277766966489430261, 1882654343945857248]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1402562167484843064,6875399334303550343, 3857176067379933739, 203627803360270947])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15900928106127316975,14602133180918305312, 3102165563723118195, 775819731091512736]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14253613308304831326,14455644217171484505, 8813780388892528589, 3428392903277964814])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8716800045994188640,17003233825149689981, 8098765417180643288, 2662771645976574338]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_25() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6553934318775204826,6448762526706366810, 4207706983568732592, 2447315086685859234])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8564844502297317637,11743513275047983782, 7106704019788636292, 3386518785120963128]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8136367708798915929,3734063950707739907, 7502883365095819331, 50417444200358514])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16304316144597689460,9593459350258650686, 17491872581202717407, 2657458824630174892]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6362178881552950286,12872059517268336189, 848317467793123470, 1938197070931273555])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3878960701417830695,17789057242889910099, 12406885948743829674, 589297640735498284]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_26() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6780129401177560992,12647643480185880018, 14224451688303834228, 3302818697791255484])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2192086844175515863,4308813603337643331, 8565589014370561090, 1641207662169117996]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16460747740013996588,9954316361922225430, 4655147402209073759, 2396528941525913710])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13000412284760428779,2976670534361164242, 5975675671421520118, 2907549454699426544]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15566946663109454185,18365734713178870860, 3119973425936796420, 1887566255794037289])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1943266906870977791,2569165664105406165, 7026246558310180131, 358469021483687492]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_27() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([905852481265356938,12180218557512563420, 14940503701232003774, 492871638040468605])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7796202633426178942,9411239814678011750, 18243994466589837923, 2273217624299587274]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16677117661200790785,12067833415593212423, 8026805563642591158, 524513103672087320])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5003345710241591509,14218176980804888811, 16897812385299897593, 1407921368484022723]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3893305850977417880,3361053604677418072, 16319271345113216865, 3178672310173794701])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14944435893153287549,15742612385713522429, 15105746466718142644, 1388120249633498745]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_28() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12510602126968752057,14411821122146075345, 14503705327479145723, 892914366377655129])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2514555661892145721,5073702870976515778, 1377755573466641014, 893678006687460905]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5851247060285744877,16788428557962724530, 11858121246987299842, 223139928112749098])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8075223826755447500,5666399395747916729, 7318512750933081830, 3117091723070705667]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12251410391947538357,15057215496991319474, 13170938247709861243, 947352799430131006])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10722664772791149411,13196339966616832096, 1193873973979118679, 1469595993609682392]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_29() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9742640854943767232,3258513333274144262, 14609535887126440947, 2027720593974016543])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14676805011451269117,11484306077351597964, 11241057383199806989, 1071247819247221170]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5300253294753381372,3015664333030891866, 6611243014066264162, 634412671585252996])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3055374770471139652,14149906249080136245, 12131749253928525180, 2476117871618831570]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6641381688194816822,2718745081929775329, 17404064962693660722, 138758528359656281])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4037854314864578628,16285991608978625752, 7921965698191905509, 299379920256125502]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_30() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6082967722225242767,6435873538541097664, 2539499510617018630, 2872807939775549755])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10732330379674854831,14561860691486281611, 2273709627921684606, 3068090580046767552]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6273938110038162821,587596628361864174, 8207997860878781363, 2786933468199806137])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13022675299022577066,16042598104084424312, 5756815851518770631, 3199069375551785998]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13604233058157682470,17477014232038765172, 33605382228045055, 529022133800180741])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([556227067338497186,12669497927733918522, 225460558547001097, 355483476191156844]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_31() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11548297236231806335,13516778277215247728, 1541511792685197290, 717032995719630991])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7876568317101285434,2687316960472853517, 8094301442509296278, 2865077876610894694]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9866497075543358886,32708544858670601, 3957341255686023240, 1669246745418825799])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1043798379003461937,1112815388387824709, 13063010474575245205, 651137018445029440]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16151703341948344972,10316272386635794000, 4509772189964312998, 2182609714910221558])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14175810481317144536,2459160199478707765, 397447626083769523, 445900574594189991]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_32() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3937747337242263115,7916940036034482657, 12528166875052555352, 217638346864287736])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2686008129538064618,15604860822278292194, 11728614449814821890, 61874964446169612]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16687346381345327842,11860932584391083076, 11502429410697546196, 3269836246748672128])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4392530305579852923,12338024090528409508, 13018789351790854167, 3349162449849925333]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9810708309885217761,12556862275846836939, 622715259997819595, 2640341840134064692])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11102969370296489718,17811293655306957306, 16283377347850809442, 1373696843989014888]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_33() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6719001809001591656,4523870988195113021, 13357701125989029676, 2450490249736616236])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9126786230946672839,14267838344428834506, 9092617862686151959, 2934310128690684668]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17252839508975999582,15662278275603221298, 4655988076540607474, 3081218119274829362])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4866900452166394427,8666565389797993206, 17831334026682020431, 353058218583872485]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12717078058150897264,17613874175750862532, 10729810638218162062, 1209792935703760516])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18239708202357525720,16465155965964236829, 2183991923782563232, 512089109840592210]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_34() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10961551488938719208,1766141481692797972, 12552562650275719827, 1826811417577717051])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5858460407232574226,9428072313819307225, 15656101322296194070, 891754612759801256]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6718277829098390472,14533150092945553772, 10468784718156461138, 2002391996926217241])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11785648932428277924,4383438325196459134, 13988903858723138123, 967905184872202183]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3599851671751722812,16320787548193543156, 11846507863342347477, 1900306656298724585])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9806941883590758463,5973524033609467427, 10421959727106865761, 1678274449605242922]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_35() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16423338878263927242,12364496146706814449, 7642024531929409489, 43816368606672491])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6591362803068120929,11464338862126686271, 4183120185160842056, 1640207266279197054]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6251760262477376773,3944640088471781961, 12258884680710907679, 2075481449797267702])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6121230879900793847,2598666755316118775, 15999981513121873702, 120156590198350277]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16995887692231517825,2248849007428925131, 5606810011398029416, 3072013843203520286])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([284051022257779639,4495144966703800167, 11308945265687611262, 1994223427392389152]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_36() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14075970181105329601,13191547245464656742, 12440881529892118277, 248743268282337747])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1940577279674804692,7307637683071057493, 11496726582273067669, 2443430207423370889]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([276604543420648962,2056246528240270520, 16323809135038053964, 1195572853205145000])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2539599838741137894,11971117393715857372, 1118545372737114900, 163193516251450639]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7668359283723398473,5177973958510079892, 17766069350504192697, 1147662249465715836])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12256786320986672037,13798337193542863446, 14064455539826323229, 1313825061252299417]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_37() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16452353793514743284,4178107869863362294, 6353875452060175327, 1990374331435628631])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4496023247414787155,7709427982050387669, 1366784166601188230, 1910091174603872922]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13953834124610205286,7733332510850615608, 4412166570362607193, 1512276631875636768])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8144977710945962432,6224710911356859879, 12025338298088398685, 2865725342448955742]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13445858826510597409,2369198280001494324, 12275885861274004179, 1837706286690975001])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12833503583431364578,15178118814012097125, 1529680956644619935, 2958030932176204619]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_38() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8973318899934416766,17771888460874561606, 17050188931626652981, 1680207719759922265])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6710733306252301887,16459750115262049503, 7892905343298078632, 547765362807315538]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7858998809389792790,14199355041766052594, 8111714065926540820, 845464244027496853])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([272198466538159418,17161081341997887347, 4736410292572891082, 1720354037424850607]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5278986483982314636,6902255253421775771, 3133857889334090639, 1060429048835728852])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15282530152326651940,1009277832534650504, 13568576330407313769, 2162015054936775694]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_39() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9101583404196462039,3638200842036604823, 8558492414192248694, 506299076799906387])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17643229163308783722,13719278653385505157, 10207679982470558014, 1843877010958277223]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10579776060303480422,10732490224317276507, 5224090939807762018, 2439064046248138338])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7430157923852726767,17174898363776493440, 12900580056469947862, 1568177196738110525]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3590322239287953201,14377033508527994825, 3176836774048821485, 2111326920829280104])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16275931778434727058,16922766618554381284, 14710724211894120235, 856366502273883380]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_40() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1835193358300262217,6101530486674442821, 404979810810294337, 2226068652628996176])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8440386013402030794,14165808570107617918, 15332845748536223182, 3098208687491874946]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14918440319855068099,3857272757312977800, 5185581873596467003, 2113964834572646977])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2290077245690182017,8267967350744380459, 18434564542211673627, 676958768791081143]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7599188304708662091,5911663635539334804, 3812313384451306902, 1211258025608547483])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([690572461385895455,8015907964759817150, 10177602386621984006, 2292187455501405041]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_41() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13124332418216773636,15142795017761393058, 12442267867721075148, 1830418645264806717])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2639045947511056115,2375174521772365392, 2157969568829920323, 2570805693047723630]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10518087503667842015,16525554627694597809, 2774927519521663206, 851847440123031757])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5363528649907265811,13973913094623054083, 13268340478216073095, 3158742079773607212]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13576614362550482565,2780894558116225073, 14978785712548832373, 2811016861187423375])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5117057907558740967,1841839237492124847, 17966815952454444359, 2157346209695412862]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_42() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17736886238795199305,5183105915427277211, 17337236538171267457, 1883073210270079298])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15000286626188265562,169094490606579840, 5134339245769856905, 1846273817784849356]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16761726432510625548,7894396605174952770, 5501478090924522570, 1541889703203493358])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15435513574567891139,6637920828140974908, 5665843572296674331, 672355844402776101]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4582437047199782833,17420030764765645409, 18013814609072403807, 947067906591344167])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14829770454689553783,1323499361318115195, 1052550280006311081, 2196656057943081275]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_43() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18396707035280714998,8913997653917853337, 15390904573989235776, 1536431742320540225])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16639536852657644875,16964934130853487511, 16256190176139474068, 1564993870727928221]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6467007011988354498,15014576650647879984, 13861025563374658812, 246876549902418055])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4853277601881461800,13948511116848926653, 10274866076101757953, 1120627315853243476]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9453344749580393076,13121107183146865552, 5292030310440958670, 3315362630066464245])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15763197408144542085,9402649321586606150, 9674953918492279470, 2638442282421846738]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_44() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3410803370416034469,12405889037748888592, 15744112124318514823, 1775572304736039932])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5904911126315356118,16382472384322267761, 15523139286877066713, 2047936119710164968]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6757519069432788623,3050730047727291067, 5550500668054152850, 316181806434712839])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9329850703732791215,4101228066155387700, 5204585976696647686, 2609846000264958534]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11616380350213049303,17994060210088601882, 8049931939867984906, 349122947505483631])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12900289764783842468,2148838701947620536, 15523220845876322819, 1652188448271106275]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_45() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11477445610772657593,18371490060199138579, 3016283648827378527, 351122706070252383])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15139690455406178024,1714040488935757422, 3985938771585253751, 624440392156708331]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1354877374192149239,2641604197976645194, 3141864359910785641, 1612456791957874087])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9439532567284467022,11175958119590698557, 808506300426488643, 1613638874954415019]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14946328112458098940,14959614065626696537, 6470376170295176219, 3218621348349452367])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16495342628919395593,17928381330384670254, 11793711096866468640, 1590909320506123127]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_46() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3502993693946288793,8648528139706259755, 1374643982716968843, 3226690460159735204])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12522624257825294582,9352660965712523073, 4531407988309973278, 1446624785579294748]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10103848103293739110,6667066269577173452, 13479476535255518958, 270823652253280757])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7734122291576334330,9338725664600537107, 15978051381477090101, 1473947863140000633]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15045266491816885546,4552519416608469328, 13793277485921523439, 2805526729771683465])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3346717263433871800,11791915168115277105, 16514912573816523937, 3447845198271833432]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_47() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17713788016128930842,15529233394750127506, 13911773474364005165, 55476579757236004])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13889873949135588740,786270921286080093, 7278492256838908102, 1403106163099569224]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1008056147220434028,8702329823358230494, 4245833987244425342, 1417870720037724027])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3763768111296022674,1635696934133686738, 17266763845042486619, 3472807431897317794]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9646375761058514691,10475779326236895951, 393160087206411962, 2227509708515537338])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3704702983800710101,869783383885459984, 1512187259843898249, 2924099100603127718]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_48() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9540652595822357352,7931532310995947737, 15315959096528777094, 3023035958523968599])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([268122576350501435,2841762780487968049, 17640939217897149163, 1885736302360516156]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([55438598031685609,697891210559019460, 2526389852742425392, 3306255227694278524])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2869570637615098261,488220748123802538, 5333105428390544062, 2280132264115080500]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5545777465758413390,374111265581574655, 436299719794540210, 1808533097715722913])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14126008035958533713,8611564642435416450, 10169255339699428374, 565197935916718800]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_49() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14607824912084144190,15314072175119368044, 17216726045678607306, 2372718759823467526])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8530399737020919926,10685468313636917069, 4880902416159395263, 2547948726368083912]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6693908477624478365,7083495873956834771, 16132906435996158174, 662662408014402544])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2196864270373944850,2736662658211260490, 428548602590314181, 868647629496961332]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13797630518459586469,12940127484470422953, 11511795236235541619, 238938083834775728])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4078036769584367751,17318664064294377535, 1937563526932122270, 2876596048098230535]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_50() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7304578644258884323,2015588862371528355, 5799310409459856550, 2412737252513940411])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1741237486196805123,18380162019727979150, 11178838142789335409, 2387234605403903975]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10587162995791042830,13751417241424849767, 15944359958901447634, 1156786049520874098])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15330832405804749404,16776848749401244713, 18218335440658389106, 563648359085781502]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1707402438276218930,14096842114429735078, 15873290044310772285, 2278342364267367444])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4127151099671326724,8369650691438321882, 8547642587047881137, 3160213469638823902]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_51() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17908888756958744284,12689157987111250815, 1657250735737237438, 1354520894524638184])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13579223640818107327,16514457337723354896, 13798343491597078601, 2234477454919723091]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([703432841241776701,1365825844626561737, 6234240160500305700, 1462960910127295074])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1916500687460547817,5948155384100533945, 14029370012506699811, 2167559192443758653]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2920180682444480324,1056248472961483775, 8685535569155876264, 1580859099435533350])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([697220030156702907,3165111369723476584, 4096795583931222782, 1146091668850742776]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_52() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16495356370675618037,2564806760923940114, 17669193383462595663, 3391017811772638510])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5111498712679525545,16642048447788410822, 9556961585068796529, 2254512243290486307]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3261218169307027275,5253418079966744002, 17014433572234940164, 1474547078840498246])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2425230741818928451,5228260195116290556, 2800606619236870671, 604377632959603559]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7687929989154029230,9019493360721342394, 6680181423903133725, 610813602199960584])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17557421439579384233,4860193675037702998, 1559486228174365351, 2681607451473024266]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_53() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6223217276302044764,16738495205630877089, 8610649253964722695, 1454281273063673283])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18004115251634513115,5358348563807078751, 6866803286631287644, 1367238717879104627]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13244474781629808412,8245417196577790275, 9707549165378193717, 1681785056760579761])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5815961653389874129,3562645256461390919, 2247247529595730290, 2194880059169876926]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16102688736704856156,6531181768957366873, 7980817908997891030, 907588423944992462])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3774729726929071707,10219996238878159536, 1639319372274770466, 2158025105302719083]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_54() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6353560697897990995,9720995750683565595, 16122673651906415396, 746900553777007287])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15344823139778889131,2535740546413204793, 16617527468115958198, 2385038744212388282]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8333968717727636365,18163955818617261511, 6985025929097566247, 3455217636090519263])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15898403955087529375,6454160209896717360, 13413903155182120668, 1740050851912075270]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8405007330157123937,11524530362749543079, 4920523743114388163, 2714312327532861975])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1001948061734104501,8378794951679315097, 6413478714257757419, 2601505007784215062]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_55() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8378754590670816631,8369527485994345126, 11650160564936816342, 2474472492620339107])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3090265618396319910,10086008367497456125, 17969468637122560519, 2394745739348234501]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7449569766187177420,14593630419718609515, 3520133634377011589, 3246091351905756215])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14689292597465691321,8053036453126961719, 7857969421800570054, 3179661258081225359]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5689882370028385405,17491991650961895780, 12561996408939076940, 680156055987262827])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13218015981557852315,18195033174624612567, 11232416881897050959, 389082259169017823]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_56() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12698510723918019002,16099742113893892692, 5918886605604693176, 1007007953547772739])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([50886886046116759,14763102072529204736, 16557631750745565582, 2585927614032276019]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15438629673320378096,10503187481613084021, 13352596816849444713, 731345626836176762])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4911131952964051670,2470827565142403616, 5297442627180335922, 1311450390581351371]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3192410096853263941,18046864390970068126, 3798974643381130102, 294804170276292347])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6700973143704220060,5225394978884445052, 10803020964503503060, 2638904170391319237]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_57() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8808761808933679407,9027496266931514555, 17525935508238866462, 980179182876236368])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3004042292168148670,11940748680824342490, 5667679388197144521, 1770547785108755956]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8931638429841302817,1910945807581257525, 1600682012017954192, 2043679094592267959])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9361774854106441591,6503427227336544936, 9749677660786861148, 574544982339757338]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14446405868663854710,17123912376358673366, 15288542852575152048, 2590044965401456521])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15880657253741053897,16481335405126779861, 10876863606299666186, 1492300961171742292]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_58() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2177523033442322366,6342318120781053276, 812526849631888855, 63321881462332092])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17560788533489310566,16128567817121269169, 14672294931068980542, 1666496242535097822]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12403564408788195897,7801352889555500795, 13018868371907420855, 1707517715093895002])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9019305851952621303,1485276478110931581, 15404435189466448155, 2195406494296563450]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3232325595449229309,12756009347883545308, 17795066490855292098, 3470055616484173811])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1147933577430585516,15763474985291491520, 3161575167111157318, 1212728044666142788]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_59() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10532523684688728491,8789265650537120561, 3699569667619942717, 475679958995626452])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16541903415815735572,2482197923094328397, 3825558837745963378, 2688534066937779859]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10708426520445548488,15497828882970731572, 17296494662650397641, 1779565686078270588])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3114062291553670878,6433815193086293864, 4878421351050350814, 304246745547660681]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10359099735922792575,2751806852905376009, 16254633742579462515, 3256605203221351604])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1257876896651207865,14388933140728786567, 15221263326193054007, 1080383963301694837]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_60() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2618290014608226198,7689323359345023302, 5964076499329747219, 826890656997962529])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8759850994570529035,8972611806239759396, 12378312760961140052, 2511265367217311076]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7961222186618098807,1947511298164397813, 9702546300833568313, 1149136436709692721])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5703075679471446607,3850754888780586016, 17272214811518023472, 678301931702810736]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16933353134323327709,11140843459705763364, 5073457509345441173, 991647585817900277])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9221990682902216427,6243320741149703259, 15198167951668341887, 765991612239204544]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_61() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16062187890025351127,10525684183864914167, 15333714319368964232, 1731467526992433804])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5661140841935671958,16174365949303204025, 4010601958768359540, 941096824646283158]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2547315225326869226,3042628735130894549, 14067424230046048561, 2333002409788856020])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15275306986820360379,7951734279038539353, 5472914823718994684, 2566320237486220768]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([534577605833514788,3955020885148031433, 17657668015191393358, 2521129122257311241])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10580655479376572851,17051539531155588451, 12801530447141442547, 899671112270655602]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_62() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15364100337540657455,13570490715768986406, 18253713310061457304, 1966726095689536248])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7856396005610568991,1370255883514653587, 13085568609031505454, 303147400165320007]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2789018714198597471,7105823372843398637, 2652003258035054968, 154694750146672234])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12003731448029644947,17498300065982786742, 17533410844958402906, 908312155851259132]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6465663350607687548,5319709875117736840, 2361171697570241848, 3406124560121899930])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3751972775403139557,18327847488593410022, 7039841829418245099, 1132403294397940263]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_63() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12992303955125266792,16277120679906903880, 18206967848789184370, 2860855405341587988])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7299600674148938026,6526458353772987025, 2614485700898663640, 3092296624106141919]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10582215244435651190,4975417320537284658, 11151817482464363429, 1539626920733441724])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18399380371179114219,11946126872503128672, 3314696957695901421, 350494510620659793]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12251457437566768338,12786000333060668587, 5052477478162813002, 3069663610812716943])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14929420585418957214,7813108333601514108, 18286044510217770516, 3101719484124946366]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_64() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3260926099130454699,17459998683216535650, 5939399557279934995, 2655835841445884802])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([811160148327639115,5772562932499983860, 3015451372437023339, 155562993150021733]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6931495812533733211,13196481405411803726, 6895325207531555527, 2465220839631607439])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16357632491603519920,17230108728792944735, 10308227422213479101, 1093785765415612174]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17550356374264373521,4507701835061611535, 10008894904542781227, 1837356095649219251])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9551137378597803606,5338769225328628402, 11752271530304292487, 3316045422111609819]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_65() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3225009520115219870,6399393179931520271, 1576716794081871499, 842350024819029661])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7935592228613500711,1957934855335929710, 799432397393200430, 3167787249587768187]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([19928202703197265,8202175632579284869, 9969099653445604998, 127628348224124075])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9163524135691986494,13719811815693098715, 8460446351330396706, 3079819877066158759]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5316947896498008063,4508462686471626681, 14359112795577451367, 3228644803217180094])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6676642948866162964,7240799969991767389, 10866893911351570392, 833171154608613002]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_66() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6141871313006081617,3976699058893350789, 361037668324097915, 819641034508339363])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7605455922180048667,15729497490806599454, 5801401762146205392, 1173155417925699011]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12040596120714019158,11654410505128700335, 8508719885866937986, 776288546543688816])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6455596694607989741,17858275832194215359, 16783709412365591004, 2950342363364208827]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18059153007377781185,4671402958227824817, 6383256651848791726, 652900134515476752])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18038844574424453352,3083634398816871368, 4815829830754173790, 3102650085999645828]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_67() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13200260824900284261,13350618169558069232, 17444415624783312666, 2546113211182075575])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16306692988271478106,5043692122669242145, 13725519115815712277, 3377445034615757504]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13087881962227987507,1051336088126891791, 10586222055797515329, 1773808003270921211])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8969345762540099530,14476101688420164723, 16594031864445574527, 552551625647554167]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11943499339551809189,15085134431155357595, 4716742241880169518, 3189477463661790374])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2701877743485114605,12508403639796829082, 2637131053281346114, 3414307406952182925]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_68() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([585093499233541760,9312338975834358267, 13920617158136280495, 3196975865796121298])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13353479521697532152,17803173509241358087, 1545534475817291402, 2354324527095611958]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10383739389897440893,8676921387352809429, 14566362767171730039, 1510047759313955889])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([602658695827344665,17075488153822232821, 1841777653547648669, 381066910192416541]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17894474732477882071,411911056712011452, 5602730894143760713, 305660624305262506])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8554205469555957881,9279461931843721886, 9470971290421250728, 2104026703249669104]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_69() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7011907998727862063,15500243626545411457, 10330951929318832959, 1021899409366930943])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1651046045107715318,8993303098946732104, 2846441884960616500, 85187080521876232]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12415084365310392469,14483933683526914513, 12519996704700775822, 1573214516955013525])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15278000178221934602,15281183536529659429, 13042532405419102030, 1594010828163337395]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12238759826327000567,6307985048355614030, 858375448619588421, 745083380427051033])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4799236627687637785,7517454369244861984, 6408257763874841513, 663895132099985909]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_70() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9516506873007504414,16198370810021007224, 4061333751468243746, 787385457787524645])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17586326745352102817,15288077320078037667, 6282171485192826371, 1198563610562646658]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5097645543377583809,13697364998882913243, 16907687626629455318, 987805276597127862])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10539101518583384803,9737332404493156143, 2617248783569059507, 2574460982944448961]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2569564409155824075,15916242105516574261, 15580890685725838557, 1533008728675075230])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7060627651858653725,17219312509125622423, 7937587492678694255, 1346834468983131338]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_71() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2261000395193351037,4986885898563306011, 2117456212061354295, 65240930828806417])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16548838491651323288,7770964202049741486, 11136323005120305800, 671742964416029414]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7170805638362147093,14662877804654186455, 2200918678791075672, 2270406830690701258])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7475998348780691649,4709040112137611232, 3102999963032239462, 2406591501345045646]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6923692805949920458,3547732755180351290, 5510578623242469051, 3063603152621215585])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3933652261519627523,7527191338540556597, 6383071777496710186, 2667702564278127681]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_72() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1329194321912689039,6121318079801498464, 10466736074246590689, 1675404104681868867])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3132639955571453866,15234854617586406420, 7065897458008940435, 80238068726856218]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7528039092572399586,12742611788313667733, 17846567495029557637, 1240342188644431423])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1710833455385385677,7413702426916364647, 11007281868646856955, 1494586006608644075]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2839774779577028437,15985163147929962087, 5770660719822537266, 3205760204845250043])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5610226317452490216,2719137973084022609, 14868704404087979158, 3168096064726643539]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_73() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11949470348298013466,15721200167844016793, 8318339981831976370, 3062876087942833511])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10513320755585950358,655531797118037894, 11469803582716540237, 2438472452545155104]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8708798807776525700,12122696930559599642, 10912056287226570023, 1395500639025955398])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12743806783022456104,8591310939608127192, 9488838640981134747, 2803030437506735535]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2765230001662902817,100897066273805439, 12029246456251712172, 660750681051282291])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17006055231445648180,3681130919480910113, 11216429244734129754, 451150138345241090]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_74() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([131268572405854269,13822580952425193994, 15037827988528585598, 2456126067079902083])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4356487178154429814,4424680139423958532, 8263477830818761587, 2975740529308486382]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10783220058714355586,16588938828241036359, 6627289082386346677, 2382761560037325202])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9621754819923502444,16068400737312004260, 2671308382267069317, 2845799753960252325]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10302260884629230465,17331504427905881595, 17442585000121999921, 2440588047538783676])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12054383346687077104,7402440984713351429, 7027509657673547263, 607492545023147985]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_75() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12007231197550900724,3312014382931972660, 17474718411318640038, 2198496307026821000])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7090699948269547211,11991026366470230283, 15184212107098904249, 2248684626103186571]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15934960204007930427,7242799596506223693, 6836726909864085014, 1773767267300083780])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7581996269205685273,17822438709286200998, 9942544918432264786, 2018284234956314956]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12492483644909377821,18091018305977953034, 15796844025413806672, 18426985317056213])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([863184063983433075,11588852145778069801, 4923493634167551985, 2036508102477996462]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_76() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3868604860110474934,8418267566535944807, 2546620126913908660, 1666123293065236506])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5678356790592106969,14958554840943925266, 2475070781360811118, 1971356452755415155]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17657078547845868762,3293380693137123906, 7129810464723253026, 2702225797683855778])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11261445285639107153,11571461139570301307, 2267423563534921129, 2144854979472612607]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5777325534126358890,3548337093744661583, 12587675559506889427, 1469821180124960690])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7689134843918521459,2036807200803291804, 10696521286155324485, 1775089691008037241]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_77() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5104360360363290205,14578019980316654525, 9028255744738888417, 1510332832553564592])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13360789901479902324,3684698776300477997, 4202072112828545370, 727538155782849596]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8734233999574385139,15014274100617338658, 12336120436391922920, 1101786796627350339])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7238253993119572276,5128054024845362245, 3711968133491752059, 2052153467067512935]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([472810987263066630,13708018452935448904, 2347795017858059407, 2679718872298169219])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7059177469903383179,3215408038576209903, 2507432655494946008, 2211932866631469541]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_78() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3166541087959183323,3603675257454092985, 16638661201655821603, 1173992476441340308])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8361021541322132864,5860686058425502436, 834189316558314616, 2265947671822023864]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13897234039603475757,16264821529979156107, 16700376563398652149, 618609926811979690])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7184493327477424951,4832315880341382347, 11160385665186970054, 3365867473046694426]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9771581108220763698,10131912168224539720, 6295624684372593644, 2886881510885273694])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13235490497556670563,4608682208464683994, 10156381924988371577, 3126796142263234464]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_79() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4070628397754814307,1033356962575917657, 11917278572647783574, 186580008710352836])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5219472338045567515,5671027967471446876, 14556137379562429861, 1137870790933742032]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9004886378896960164,7645114512937204461, 1454868612254113444, 1463889028492023836])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1977106499527847370,2532586895134784260, 7528249413895129687, 1552607158786569199]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15067877821559289134,232523301416896605, 6053566335182140185, 3477248413687441580])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6992024018225659472,15655607613187744549, 11873615606263707650, 579634165600246687]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_80() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5811191173064485817,10048308413951437334, 1569557340318214892, 1502046133990935485])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2753937279205837237,8352155244928122558, 11514644316126099171, 1858463037834610263]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15057834056574810405,3733496592261958341, 5565765954712024862, 669609177090403942])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8994816399816856325,1734340205565579762, 17203169462444977425, 193942583346860456]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5186184025605198613,15467409492704516335, 9450047680069453853, 1890469838330258284])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3625372670653833702,4460903324738101711, 5560925775938084881, 878049946987802512]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_81() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18086707755326988959,4416722129908800058, 2300882467513984440, 2287332909829534121])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1944113833244810371,10037080422916928888, 10845063699231349142, 2773038564111315588]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9479327475962381446,9459288866944771986, 12907550394550686500, 2559097008066279423])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([576251856886267931,11995642849639976823, 12716728313060120273, 1537564144969434500]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9992654878487162802,4884249139846989765, 17793957368757547835, 401776339160073778])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8138849140619142027,14564420536595216071, 6961846162172263632, 1169278140465592887]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_82() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10596091658405582960,16481657419332455117, 15503094597354089415, 1332463213270630276])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15242753637489640766,9008824169586484891, 10531916973235526353, 1466382640803915989]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2488473995141719530,7308624610931693880, 6935952902690778619, 2561489840343015450])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16706304779479539722,1912376831067046430, 9090192661744498610, 1339445832259158870]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10558162373501382078,16318772540269055355, 9539315509289614240, 468106131963514925])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18188885029647739042,6974074190354080138, 17378897081974797341, 1498469360722473990]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_83() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5885835999548730877,16205946095367074490, 16288968773730443140, 280465154299133148])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4983815211049566791,2093813001692102094, 7628681671204546205, 2414736470985809323]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9998183993762166899,1729784321055208259, 5271197837269781985, 2019661100404301505])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16372280390324968375,4492927628952592220, 3134679874952175956, 3084771613165054185]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6195080087702758046,2581669067018103326, 6673954319387025800, 2825012649632766453])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5966069916503759023,16567632008477972120, 17116865540559762301, 624834774353813684]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_84() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17952701949830804529,16457880609612515425, 16725812230621628984, 65036502902530319])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2473958318067800610,1481557241629741807, 9891484481493580616, 3166559868881210952]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12481780418538688740,7346318182663503229, 16756356560134554318, 940648762025753145])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13734028706457950169,6985893453636851530, 14998695711860680248, 487244830664591450]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3563062404520971357,5584357006519641844, 13575100296955599674, 3332822271498499161])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4159737638478175637,10672263664086927875, 13129061246584374614, 1421913847564083255]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_85() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10548919291079443884,10194740414793236255, 15525818989151302507, 3358470201128857905])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3374288308066201537,7774034434789843072, 4862165730236368803, 3359777353772504631]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13839617924263305657,438798725123178467, 12969634062907345127, 2279652161038119798])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10279227162754770840,4781359373279763180, 4760713668136488702, 1859573854598084654]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2632661308287166409,7139340114162280547, 18379012227656586168, 752636113797767441])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3617416953404504468,6709092008045651942, 14809945313708776528, 3285112409640257190]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_86() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11091364948516876536,17578964615762120596, 12544167026290646925, 687069926861381375])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2968962567879719960,7090732053493365525, 10543919459905452336, 702629889484827443]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([136153124187754218,6134998333808818115, 3866368159441588682, 368715462983975766])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([211326872012303484,5012029749820721737, 3203693071417792240, 1393264860716632906]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6147626535171329858,11833327324540009202, 12237998995850789174, 342239878445786822])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17298352312996630235,7722869205888623524, 17626027119359041048, 2727417242254563793]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_87() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7785470680802730078,2669257574234341433, 15863590168533596893, 124237610850580215])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12963803336082389122,1652588953746171885, 3535240478940717262, 22212324896236066]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10961972362621712595,3148604586284608158, 12820844756538117016, 1689436418028993990])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13284545839890512902,5203022430357716799, 5709701029751308915, 113493710081517099]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15320253176699724578,10002214666593654098, 15251069202607702333, 1349456826037337756])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8554628909645654613,1953062248982800318, 8218088008233315532, 224165768168460700]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_88() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14625589974458573665,5333819658967997308, 2421648155019645088, 2715570480476707753])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9974476341632538371,7705417487773566472, 7020246529306521098, 3176754048291912809]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5351093913505463381,1731766087604708307, 18370366563600472741, 1238376521537971886])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6754027882385985154,7271629789195271766, 15327980463722195734, 1311050779559793]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5008894676148489200,16616937084149581892, 18401347639570799064, 849902699894494976])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4366159503697961755,12352084736641692034, 6869831260484935396, 579800714537184765]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_89() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9367373421497535588,2253957477957576172, 14718623243630494687, 1087398560818281961])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1222209794994824516,1190463968896903952, 17991461905425444356, 3271472836490840731]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12280456167167161173,15074591995017109331, 7927486878090044414, 2589459277969991746])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2960516687484716390,14822519684082368376, 18094571197128530168, 2115823168266108251]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1121428646591704035,3289077916856693367, 785515970300296883, 2858566197442193922])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7894769704641227858,931705225557700857, 13369048121353868950, 1942397898708531847]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_90() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14777048180842232315,15579796062024126139, 4760352085135203381, 657811865174758869])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5774378405301498162,3407739767388291623, 2879889741847450210, 3156933521967531534]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15731486479318862649,2272229697208999338, 8852392503927918081, 285553289337135921])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3743497068387263668,8476624617548449823, 8870251479067216512, 2090447587834227015]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9080728331729496335,13132285493436955781, 13226131211158348837, 2874112677311832157])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2384977564215114399,17887304044191584648, 9075612520145550978, 1077946565759442258]))
 		)
 	)
}
pub const ALPHA_G1_BETA_G2: [u8;384] = [81, 235, 2, 213, 214, 49, 126, 156, 58, 35, 68, 88, 244, 138, 86, 129, 68, 10, 98, 163, 33, 124, 118, 58, 70, 248, 131, 191, 60, 25, 175, 29, 10, 254, 132, 254, 179, 231, 130, 247, 192, 240, 28, 231, 119, 196, 94, 145, 208, 223, 176, 66, 139, 105, 174, 192, 195, 173, 198, 98, 36, 135, 93, 5, 39, 189, 23, 85, 151, 140, 234, 244, 50, 18, 136, 250, 83, 54, 210, 129, 136, 137, 27, 75, 52, 117, 126, 233, 239, 57, 94, 191, 163, 159, 153, 41, 247, 216, 145, 116, 56, 60, 77, 14, 5, 198, 40, 70, 79, 121, 253, 166, 242, 217, 10, 58, 74, 204, 149, 112, 39, 200, 124, 112, 65, 227, 150, 32, 27, 162, 184, 87, 152, 84, 117, 11, 165, 89, 27, 82, 194, 218, 174, 237, 103, 106, 75, 15, 97, 217, 155, 158, 252, 207, 4, 199, 5, 79, 160, 5, 77, 225, 126, 197, 181, 233, 217, 75, 81, 169, 198, 68, 219, 165, 144, 78, 9, 117, 98, 138, 172, 208, 108, 0, 43, 148, 99, 29, 15, 199, 102, 20, 96, 100, 105, 18, 178, 110, 106, 76, 204, 118, 50, 99, 98, 81, 58, 165, 9, 148, 130, 114, 230, 107, 184, 56, 73, 77, 198, 180, 13, 72, 12, 48, 149, 121, 87, 150, 114, 176, 163, 137, 8, 81, 12, 32, 116, 254, 89, 153, 159, 143, 59, 111, 154, 193, 95, 84, 55, 239, 67, 78, 194, 183, 154, 21, 24, 212, 9, 102, 140, 168, 87, 8, 202, 221, 223, 17, 67, 50, 109, 35, 81, 14, 217, 112, 79, 152, 148, 137, 126, 83, 13, 23, 164, 29, 122, 39, 2, 99, 109, 171, 192, 76, 4, 121, 228, 232, 21, 122, 153, 92, 238, 68, 173, 44, 77, 25, 57, 123, 196, 104, 37, 246, 191, 200, 127, 185, 207, 28, 49, 108, 87, 142, 223, 74, 228, 233, 39, 226, 28, 58, 247, 48, 11, 242, 108, 73, 116, 207, 113, 236, 205, 227, 66, 33, 60, 147, 197, 98, 56, 35, 245, 10, 138, 239, 222, 16, 211, 222, 178, 113, 239, 24, 109, 62, 197, 154, 100, 137, 247, 138, 140, 135, 23, 67, 59, 61, 60, 227, 172, 201, 148, 25] ;
