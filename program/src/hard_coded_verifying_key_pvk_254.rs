use ark_ff::biginteger::BigInteger256;
use ark_ff::QuadExtField;


pub fn get_alpha_g1_0() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6395671874356304218,7442367705146823054, 15233428457291645462, 875023476512628524])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14555270887283425686,1422345715839805534, 12284638019070615609, 406803177789034317])), false
	)
}

pub fn get_beta_g2_0() -> ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g2::G2Affine::<ark_bn254::Parameters>::new(
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1908919010654538595,14236075201559531865, 3691261556633759387, 3181789518742718036])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10353321052689391367,2214908233606902394, 18270478741466760724, 309265333770134993]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11817568029857600503,1345620474978798134, 17042686217557036156, 104499137409438257])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10979309571565419044,268767246026620089, 2670276847661099355, 1486146747370705440]))
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
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1979133458926270107,10656020283969291486, 14425184455923534490, 2371757793303704236])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2338779600581474015,13843434437986505840, 5014726876714636860, 396338526999056216]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12160503985631267562,7994430980094935650, 3652135768443739469, 1355205450915283418])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12501883721587831322,16014653649663568362, 8111577474274034200, 547525166547097671]))
		),
		false
	)
}

pub fn get_gamma_abc_g1_0() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3159389374327072544,8702572806627812644, 14735396030600707426, 1415882515485821835])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5282952517879677363,17589495913141018583, 6276679321723046301, 46207785573577372])), false
	)
}

pub fn get_gamma_abc_g1_1() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5278720720610482247,4853047536955853704, 198954976969445539, 2390147684284188656])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15048582103063936163,6946995961699431642, 5437103400516362000, 604210294802098072])), false
	)
}

pub fn get_gamma_abc_g1_2() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6878141752388287502,10261920659202884658, 16808027193156234082, 3264255560936529933])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3239484426315269233,6588951232582622021, 7184520640332291944, 416186791699681882])), false
	)
}

pub fn get_gamma_abc_g1_3() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1681642874311058286,113793138905856008, 3081611658645665029, 11158016856745095])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14781059525987296718,1297539854827781261, 9748381039933110252, 1877218473523744484])), false
	)
}

pub fn get_gamma_abc_g1_4() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1846904250284650123,10966682437869480594, 14713576516186106345, 864687458096386730])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14842561321168141556,4089698798373378514, 14319118631512416002, 214536774955682222])), false
	)
}

pub fn get_gamma_abc_g1_5() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3453320975598334470,18390698318347227519, 16383099553606031523, 2466761940348210207])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10439753338470818868,18284378454928184376, 5012784863114104990, 571220535945855967])), false
	)
}

pub fn get_gamma_abc_g1_6() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11812136711347424284,10093635689677075809, 3247718340162790936, 1126164581369818892])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10500566054385294176,15377675431943725633, 5264977574013863271, 1413908097804545386])), false
	)
}

pub fn get_gamma_abc_g1_7() -> ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters> {
 	ark_ec::models::bn::g1::G1Affine::<ark_bn254::Parameters>::new(
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2126405381201639828,2643463100873144193, 14959479868134822333, 2193578579080567078])),
		ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10089167708472632376,16054668045155295188, 12823131047465308473, 2143587365160204163])), false
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
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5874263897552983508,15988861960189871301, 7304271536887478938, 2710410901830566836])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6557023369466111028,13582563225617585109, 16223154948548068401, 1095050333094195342]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9418633567499203416,3802113722006085966, 2862966600781797861, 3158149781857181292])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9112341414457188823,14959930202828100067, 11361568552776201460, 1693636031974294105]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16456598869599651326,6046671751890402142, 6123581617055427194, 1847001181655854707])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15576773426331101739,13156589877028162109, 12791818398483746979, 3344372742731208367]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_1() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11771198021789967618,18090387797102426348, 7511999888152210614, 1259397532550167296])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([735567365032608513,3071232600740563881, 11703554310204049285, 2134162137624152965]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10853928756768867284,10423286779942086059, 17074830482860360061, 391697597021570968])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12055212374730877765,7966785331428188510, 7748684549802413882, 504108346809950722]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5167419613888609943,10266727458138111261, 1945086642509292739, 595754260877209410])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18101969747938735315,13383266567206768616, 12737346714159007339, 196579388050180334]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_2() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12895558265645307017,13634865897839875322, 5033101671834470322, 1177202329729050831])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14477002080128093339,4966184429446556411, 4229580148163208547, 1603889455467720777]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14384601202423635332,7113371871079643338, 1070282085670688604, 914361015020519320])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16063062314025262536,15379461251364828712, 3345372191994413518, 2075894975077720645]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11594214101310627314,5091772191303790989, 15340090705594367451, 787478217975220233])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7019231450457676260,13887440863841722427, 5449257941155343878, 1617815125152971557]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_3() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15117371278602703750,10430612015237214413, 12393448984868580539, 760218948660696317])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9416822397512469647,5572901566690307454, 1179440456185668598, 340720477692962326]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2796086930358014066,11536272004860655555, 14054913545234231488, 638479003479400121])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16154724725619528210,3564417695208673743, 12478684397780662087, 3393514101193769679]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1317629712088651795,18280728248924962539, 1006325579609739425, 765385185711451176])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6655427544433738932,16605451050199330352, 9468261594759119652, 2685240470726224444]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_4() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7094289282131085965,4449755253571532751, 10307130930092857811, 2438297493742683897])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8622254100483158284,16810298057716626310, 15252884013618169245, 829886272642454611]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6484442611406069541,10383507904176567954, 12779593023182776495, 435796825562085212])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16035219220638676822,11178572877653067893, 5684473856150168322, 3278052198898025059]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([628170302820519241,7658139064818166194, 5248892035505740299, 1125102399592791622])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6891609723533975102,15999883303079187481, 11513039275015278965, 615830256065894167]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_5() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13252914863267605660,3498318948200847410, 10198375721042984633, 42461713911298668])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9828921300729275005,17948773650116882065, 17642958854758114525, 2313201775942343710]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5574430445030622733,1565246867589830231, 7210482273133885110, 2536209092639018327])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9011966800779472129,15973845634885233016, 11118431923267175716, 1371164745658391976]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17525654024893072494,3937514416765629216, 3460601698546619250, 3316438261797655847])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8385558868578008013,5134258607127734559, 5998775087509479607, 1422325134324495045]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_6() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8136865545817402014,13102240821369314016, 14149635758967054913, 1092631771163757334])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8039396300227132135,1582924839071725885, 17232423427363040607, 518480628747491258]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3352657507554747638,6075377625981781825, 18001727296289648637, 734352941619860003])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2386067369412777963,13159272980216487977, 2245560089181105390, 1832874483792954231]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17791034778004771045,743967401843087036, 12741709891257839778, 2369370990545132349])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18072729254030217140,6141523848901688267, 12485947103892287824, 72365711219140055]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_7() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([345011250578299096,5206477008011274228, 2698506030721744933, 336220765652489694])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1385968841674904063,2758035792717063873, 11808450684692826026, 3076125216383720906]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11998728475928165907,11813783368660703270, 4540032169509956846, 1856051043312227075])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14235150206231271259,18327514162013954761, 18208845669758724660, 1881784840913952735]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13715294309531080262,5428136074552370077, 16362727206451592699, 901125773158611788])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6936298898013429202,15509965307255703733, 14740411387545909357, 1254983966524045693]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_8() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4708661712966569630,16596873207459905376, 10777755036473433722, 3081644418263141816])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15338542430649525527,8555870418358308647, 17611242289576677129, 2572143683065167846]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7623840999885425590,7488893902591169938, 17051809669729856056, 3279649979043912105])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17876266438223564188,5816393932130531638, 12407406303313917174, 2562231019038708926]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13949068614292413904,15048783265997480938, 1980369422543781816, 2388383785575959384])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([343548084007857316,12448484742675393911, 15019677945790563326, 2258381780881849002]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_9() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10290720339221248577,10231935431363450962, 4442192319889943000, 1077603219015327582])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9677023237804440220,2030733589272874595, 11975487399679731686, 1757265167564781215]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12132604875874891161,6632233949619769740, 1099946751590121378, 768246719395891485])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13244477407634093094,11056505608052070534, 2585639706257233742, 1101442569917536712]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10225944567366318958,17415472043166406987, 3470232320598375869, 813326555377930762])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17230771312023800664,771286161470803666, 15917524897454663162, 1390220108444427962]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_10() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12141445980929705053,4407097253258288623, 17388807972493070002, 2712044787079152063])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([106024051733673078,3265379355311457576, 5669337191524969467, 717558159795401099]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5394721779123724577,9338965027637494057, 2665112246503134737, 3060508061202781601])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10038954811392369142,7746665215268674114, 14842276718337070512, 1259818878277346895]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3623904545613619087,14544413267209356820, 15020022409120569133, 1327177845373678362])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7072117008349458645,9928725573162052985, 2227594634813136243, 1144927542992279322]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_11() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12863623999082816780,14041911993179013529, 8554795538681263708, 2900995117729013264])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17082018353373446401,16539694137254787995, 7058893209962985418, 924130929591079095]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12272792093392050776,8547211742789617824, 4494274506183432683, 2692912545766162881])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10998771006662311067,9703789216350208857, 7158376528878247472, 2896118235157445494]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17750808734299207572,16455955436934721857, 13943121012515640708, 1197200278097655198])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8701650179099854761,8160623245731639168, 267603277307590301, 1308666300867766575]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_12() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([221791130684831527,14730338181667243846, 51504306967635547, 1646794122629372677])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10343962928263448197,17281793893358674293, 17281817604000182302, 1610314433356238175]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11157028936790487502,7086743430659166320, 12622730687094344322, 1367682632135493658])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13054757396609938652,9710706645424106302, 11014703638162127800, 572679304975916100]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7796098481798563054,8360052766473257327, 2959868308283187129, 2988571547764783820])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17591151016917384839,8189981065231493748, 12044500731137396822, 108088833151345030]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_13() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6984802003981450267,26280801655749206, 17771312089541170558, 770969641827996717])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15472526144850550876,11650184822794226345, 11757059515148639573, 2881302351636059453]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8936354359303822702,1502747604092601046, 2989295890345940616, 424637768213475717])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8984754355038775066,744876399844529658, 4031066325882396985, 244882269438073370]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12914042810165481823,12177706166841961070, 1872891441355773059, 533577008594742091])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12778413597780068768,15815607890243611706, 11289243816234301066, 1185448824215768376]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_14() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3275215018923739441,14295581555278040028, 13687336087779134106, 588527542395030118])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([88921825551561911,17462795791594586204, 18105043498507445163, 545672445134812852]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6143849322640425266,16117905515351877974, 3990127632218117336, 2396093629705869534])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8693475069120493391,16990672316780195218, 13924942532359532570, 473538604430272473]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9692022408767013466,3150689896014472501, 13165925286477335478, 1121589103772169089])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16726480968196569585,2012844031384681823, 10896393151491718294, 1134282525295089799]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_15() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1498207411340307100,9362510614147668576, 14736785756461693658, 2423049499499601349])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1390709738349131699,11774513692650865409, 11860919899945836666, 1086654611724926363]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16218957804765110659,10436190330903233485, 188357976107325135, 1059790767715154087])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16688364641407690996,1899800716647735006, 14446356500821039863, 2375368166338774374]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1097587752255845506,8601985911758044694, 1002095831744853807, 2503677379534716029])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10294358310550881981,14736122008470363670, 11401834018981615286, 2744206692537055494]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_16() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2052412380068438662,15921902957112434630, 8984381199662850394, 474324717299764246])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5336952331491470929,16672241450943656723, 12593040455502132289, 2531346310858964006]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13842444164356152939,11977405067713987169, 11142026646910804046, 3473244198543955957])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7616945353072584979,2363780654142970599, 1069835088406762154, 1179487096278285980]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16184263063497983973,16342477344363345403, 5201373699420771217, 3309128839743734009])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6668346853056733845,10560328871671476531, 14548349126194058564, 1152876380940139405]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_17() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8853791557794626485,17185137616757559287, 3763840597877071737, 3178666288337809208])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6243347123825861724,18256247546716759296, 3071794562985964467, 776614285441194083]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1180253805126113762,8357310867147997761, 577947296374999948, 2941478750147587279])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9347119981158262213,17843175748070443442, 13026358523314866836, 58726904078563768]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([238340040767245180,9756131505456627946, 5193924517538990557, 3325679902092650351])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13063467047132984776,4713447851161435761, 18078790134356713011, 753466543057651600]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_18() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6008193107070640649,10226544734488719338, 4390806097903099010, 309607627745342160])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10615598356886978168,17452469619777498790, 13356692854208637021, 1732177198691806875]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3477264927075116008,156624333541600740, 16275266308704704447, 599359486540533866])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5766627748151082238,13658822098013829570, 1321839248800126385, 1610118211905290455]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16794510282769573123,744731118700932450, 11242772540488937533, 922567127877470190])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8671106671268919335,8794085372783913220, 10013084683610482886, 292948136565563105]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_19() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3706169638921712183,16563897195433064993, 6402494587557491856, 578078602805668241])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12326512549539988814,9159238921596523848, 5291581171175929024, 2132015828177891380]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2997613538241559684,14390182298629625630, 5240672799846379050, 795748756552907967])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13833509110679434041,12379036536553009181, 11457870540743765063, 1256866254383475071]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16975007215857174794,413492658207164039, 16892491824513479839, 520745140698258895])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1630595632321249882,17818193948546364685, 15497448439523292367, 2579465045391241267]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_20() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15708608758941795482,4572085747778701019, 5058514907352226092, 1968086243222504660])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10113433637457775568,980882093935435322, 831521655476627096, 2958147586372685698]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7394688042327729427,6422041240193557534, 7881436634794994459, 535694322021627214])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13657798921824705856,9790798182477019695, 8165917385845531054, 1324403758365883407]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12793553819223310919,13881089453286949725, 13456000023362037835, 2057156289899923748])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15247860924166268698,1656084420196592624, 4531766207754290861, 35994181189132293]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_21() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([637543703283310475,10280222408653644951, 11028309828491025507, 2148099117327369404])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12920408670465024964,9586892874294105535, 10865222511812871995, 3045579299126180696]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14466412197276719255,13405538883698673232, 13037286934459107424, 1350048956767659648])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14478514854498461029,5758374211845744130, 18259421055985504294, 2808276713136646009]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9317306833073960166,17420037716969208498, 1335786837844070547, 2086695333415722456])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15397306471764751825,8894931045246173462, 13446129286105990048, 2236135252679319955]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_22() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6214825204560571906,9987995140775194077, 6413904090915776170, 2645576493769528654])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11370798398362574414,7048260443742701523, 6559590863298308195, 3260489069909061145]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18042505531716561160,10646635849078260370, 13871699623167439124, 1242487390786472816])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11374023989319472642,674421447592540550, 9598232045796751356, 505756151647948512]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7196352046641172509,2409401173166344084, 18359733372168941949, 907196309202311168])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14132230073162803533,440031012936985563, 572785104079210204, 1532809937615745472]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_23() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7091053805585002576,984392992892730156, 5029868441497167684, 591542371170623118])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5558086535078288334,14944282099137395162, 16034831260860077345, 3124176749514206260]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([204221352893003243,101352803063220709, 15658453091898999669, 1647651383720318519])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16889045867930211528,8834203599865083622, 7303083381428771948, 2554954857943727947]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12585921177863170508,15995484452922579702, 7565334027726922564, 2742154500308807020])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10348320047415174058,8155090772488120063, 4863182024277442196, 2466688108117342513]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_24() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13950033665763418537,7745066824070924896, 6351983220743499339, 1712899361083941668])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1864284990477213484,8809745380762488791, 10580863932449871468, 422115119249348194]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11558183430844728635,2116620540266395967, 6012563947164043125, 340881087401646223])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16463445584291774371,18083261854085397567, 12236738461537043241, 1882310960092142250]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13439140568132751131,11469392052958550223, 8366187514860428853, 2342922207371882359])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15429113968046761165,13976106121702917432, 3918482462647656058, 216606385543863640]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_25() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5123133906688496935,14199581252438965926, 11692920710179452651, 1251561152405924402])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1703118220694330305,7865939224530285563, 3708600535775288056, 2861326752121225342]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4797632233236457504,4189576923931296479, 16426320575535630739, 622146006181274293])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3584015067933951017,14076666897515895294, 3941095860058476874, 1246019827215376458]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4279642977480395785,17795334212109771845, 81943152332958306, 3384399380929470991])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([142386957751301839,17498849982991411472, 4595741232163084187, 1551129949118870127]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_26() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8462239328934373616,4226112100236350196, 17898726781165343700, 705324449067374868])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15654477551352013110,1231647886295089480, 1877038984919771922, 790478222687852811]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14406177211968203855,11128039553779252468, 13279670786485931903, 1064564692017228536])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3347029121657339437,14737844666748295439, 9221543494413995177, 1308463784685254884]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1366233332151886551,17040675974289935438, 18189407766621256907, 809452864652230352])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10939788625453508933,1164185202778582008, 16885719222254195935, 2119326062869162446]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_27() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12764817852852054670,13483911605986785281, 17864494791041661346, 1104568856034145210])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1048533108612818749,11001823666220352072, 113164799691900692, 601352980637055738]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14951639948243699045,6605958923628359766, 7267160557243795953, 2976405096796944075])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3579793370379758789,9922827998640342705, 12919877130446303864, 1085254169477385763]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12150646199173196331,14544800492776286006, 1937586183827087406, 3082723036061552683])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8541404079007909457,12018447499730239358, 2391027763993037219, 1978807595702634271]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_28() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10020570440790656,8989754589741894323, 5978570715778998563, 1436209732560577993])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4769257561471023871,823963144869596600, 11107318574356501395, 1925848801493079639]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10072405261513235433,14700669684683126351, 2064428038832764726, 3444732457050974181])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18146085629628625864,14611438335256087321, 9676937982814227659, 2731402603178925006]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12748407209787001771,15664601755691194590, 5087144019015986571, 3007038646639225152])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2968239182668441821,2475810645670589754, 12316525145858111816, 2307539390144693415]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_29() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6708516814317563851,16651008727921021377, 7461482053367325221, 2820048911190137282])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12537445957595059187,7882765814457514169, 4106763213437359540, 1330320959747786037]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8051127265590879159,1220809664116235065, 4699060272469545957, 859942021406836216])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1050835382799128561,9455997520625545293, 1090761761043057238, 1169379669507576270]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14047447842765862524,16927012097319525386, 5730428970816605457, 2330537455596145433])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15533015848509421475,17125391119377648876, 11640064901614171741, 2941610612856419662]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_30() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14153981684341315712,6512998319458251482, 6153064178433454338, 554983721829758965])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13658678205892906118,676850606626020264, 12140083001221362361, 1390819332677405346]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3397017444665866826,14432366253487498902, 461603164686720213, 2533113002291169684])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([760272195831035937,15710477990360104265, 625018901819336817, 1570221955393496901]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14313451145764106698,668571608085292407, 645222586485871822, 3198427795488406564])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2673951700052881917,13412372872609788459, 11845522131497434388, 469357153190468259]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_31() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6392670091221126717,8760341391387020190, 15529938239206697314, 876119636117392187])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12640151509954724227,10655986575497586827, 28573992429444511, 481027044912026091]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12264826519962565313,13418266799070566432, 14285017287372375298, 2460194918443503424])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15321633343481131852,16000216764999136147, 6509106322292842401, 1406489483258276428]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17477267749101408590,10979409494882229760, 13661067527493356922, 3115478407902611511])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14741594669932279073,14001552366791225780, 3041817479575990265, 1497011385895211386]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_32() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4913881302055662547,1147344199258599421, 13588603960467699215, 796067331306270756])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11924023979993175253,11869659374707989780, 636148056215124678, 1721714959011760004]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4586400306065376701,2228381564639926816, 9951738429627888131, 3440963149526181132])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3113947929089437209,9651442724177151061, 8209548111867073537, 1801572123571979272]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1724867553704458001,14034977822506197943, 16546520594310157675, 227900798107895225])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14022117401266403012,14123788721352764977, 9884016893715936357, 1855772351877567194]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_33() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8718857944865157633,5584087750137160342, 9537114918544924534, 3103645198081681683])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5799708544008585849,11659012658453098560, 13004168647074694834, 2850726280229199175]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14998676630524370306,8821857427018428186, 1445378758511669794, 1785888209515287773])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16941511093527000167,4663707152944150029, 1155380833652927775, 1569854244867007506]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([840906565930912073,11759349716930934079, 7995997698435117624, 1623888250609313733])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18172149279897696411,2215351769953040814, 17639049087588264454, 2887227476606460448]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_34() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8613569506242897960,11138745120942757604, 1050638369929023150, 2544764751983362778])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8487681120762449442,2257964168451648469, 935043031173459610, 2270996870630892440]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9934501686230114510,13047965959595863473, 11487238148718107033, 1348352543431619283])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2924129698542120805,6445582034726566486, 6823246036518069509, 1895034998600713323]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10221538894843392972,4132229979043506387, 10604086004992895388, 413977233894641595])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18331695443258240505,3382060093242858147, 14450586382625779013, 959031320859992795]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_35() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10239048679671528260,9436634327637316873, 4295389450159848927, 1435856102578282783])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6049790219686072540,5264743454069675360, 7532990180253079082, 3206699097524475893]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9609458504982547006,12738455043423487401, 16311453986204012489, 1356321039802894921])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8529495024385549812,13488658706077424399, 4735495015208262074, 3261125150292991923]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3605613773737533405,11203150605623263111, 5345692205599904570, 1004026011216667160])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5294004864565386594,5722921593433687434, 14709233579939881498, 1134823800901782713]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_36() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13229711339549584768,227899061454841686, 3695215911625283932, 332784973710563763])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17736467673656948489,3014984312095589901, 6286472130197406191, 2336751364327642394]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1719217770421657892,1552486315484514796, 8576047757976160864, 1275758158614181059])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8646775644016835161,4658472271221360326, 17728375408316792705, 751176883337886065]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8628019019165282012,17489136407033297101, 1308042998625915063, 47659198998063407])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11710815517306430554,398803014645254203, 1664767167038479944, 1107638991229948238]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_37() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18009098213463241656,2180818624100385406, 2697150221523364482, 402192777548641252])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4094484478012985288,9136377832611805947, 8504786447479233439, 535660468697549373]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6817172513071262011,578476011248592161, 17105158538877139844, 1631917654234827828])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14776217220407743232,3688952104006312382, 6866772530510522145, 2607551527420633383]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9942723848186459179,2639789968595703411, 8740461138160476122, 2660097661364200939])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11907054133174081421,12440891103930437437, 4560784827751401066, 566799646486044139]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_38() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2596293313085999664,13614708563502154281, 15244333552013112354, 3310280407289713755])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9769849744058870823,8667766653698707252, 7312581701940029245, 1351107905705317208]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11637722940608938820,7735168399392445361, 2690473160905388409, 994972718272980740])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([530827976888090309,7418405962318124610, 8308361404310854331, 472439169294339050]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16448641727494815684,12385347370198006627, 17182310379402303829, 2441363129358426047])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4601007107731581710,14573798714447868552, 17959001124057674964, 1327448757200100494]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_39() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15853766954122503610,18107642391025541514, 13867586560038366828, 187366278594112674])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11039261429494604589,2156202985499020782, 10133693771807560973, 1493565040080045630]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6904740693117061121,1884074898180916166, 2133576047668859549, 2835477657377682760])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4857880347403304372,7304109013465696330, 8215517535318682821, 3243007133922559325]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12628369133518752016,7665877249504458217, 941828776036225735, 675883656488789197])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17052183439213865910,14475130511911075768, 9499386367295276736, 1470239461161817628]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_40() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1439589497338598040,4908137368223809517, 500053465048100216, 2497455118301213823])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3392716288543616074,17743060485039207752, 15966145852635862209, 776289149077847325]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2300840609587949857,11187138777616078276, 17279072205701972882, 2654770293859440366])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1436509363253446924,16686605204136163351, 14832506535605077382, 2948427593427055009]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16125905028831566103,6839942250981016126, 4899171655132200329, 698552853231392676])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1422132399308900604,12552835270226386411, 16154192774136829568, 3036514553430654045]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_41() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3147377741893204385,15860286534931652780, 7340248713607843298, 2058573871503908276])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([790548409788850278,9750271672111545594, 12377263851868795289, 2637777803413316240]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10315793219144279994,13962715913192154469, 17763209443057493035, 1458822425148284233])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8840369228629175750,8987317901750395352, 15943189925423159062, 1205581735582413960]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7512831314726917495,16830866311734440342, 2846841622502598835, 1115899505655474346])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([573479089786593392,12037468345328639063, 17692016579707381878, 1830513828346855819]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_42() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4094658137815112327,2411435147143013738, 15362754508534239230, 2411411032633384319])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5642762862580609134,209231750844383581, 8082223106172606439, 881417372005192758]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15135893745435289715,4441726297329465694, 12364734958559055999, 3084347312499354358])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3691108805973046125,16494967049826274070, 6968165891592756344, 1278850635415081470]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11535066671878299461,6954224565712078438, 17327145572906735769, 712847326575172686])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9990014088202463918,788626950300134776, 3421184531683275921, 3033671667589501722]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_43() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([281984612052915754,4041523846084414423, 8490529169749582862, 3250474381225235699])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7957416140320226048,13242247449757883119, 6228492921795999753, 3032205022195687812]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13286127676908260636,8352400314793039032, 14737808051217398437, 598910539084762907])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5363251321631546054,10068187124817266812, 5045383212998630447, 822069348765844593]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10388658279177191134,2956798204996388914, 523574801962445778, 1625170703517240056])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16423201130315833031,16745346721549997076, 2259186136352385668, 2045473965463549620]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_44() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9174768653110422569,15519705132125433110, 4317037715410390358, 74579396697502776])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17915844008956429641,13952504530070425261, 11839351429216409888, 2829060672742950528]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15367682484260647178,6212475801478421555, 8790729819486707862, 1091013475039504730])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12766609185019621566,16991576349639005499, 10774664176853957622, 1960329796999908387]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9394760303187272353,14003237230975813601, 17586683296416570689, 3134704245588938020])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4387320204833111906,15112399274567735280, 15769411382121380486, 1270652004219216623]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_45() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13608221331968013811,2148619124859438334, 17605382042734369856, 2312463762845694049])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16892965933949918722,1435901469327104858, 11653911995443451424, 1923304122289426824]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16763375638274475157,997919745510676473, 2498011770259298910, 536697255953368327])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12250933781881364349,12205026598458900476, 17773577007243700047, 396847027710827733]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1807428726076327409,18272549991611671778, 6585035232392826774, 638962288506613288])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10014324436650701033,15491633969824561142, 1345648038160272130, 1369352014844149064]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_46() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13404947946070908439,11238167774694696517, 3363897949557040265, 553650800100666051])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10252913552879603847,6899088663422921744, 13974679457957665998, 649064723310289538]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16538867559175481849,12551084995234249747, 7007782787290371483, 1156976375650985609])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12396337047236350976,6331346990169924041, 5651427521980448077, 2498712675830625566]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15746301974216871405,4213955234394717941, 8457505777189754925, 1389532607732654010])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3409301283870867948,4223414195917630239, 17302378335672622367, 1617922390396114672]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_47() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5614910194664927792,8595727875309757403, 13487985894163991889, 2056913501155533740])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14739004467634668696,3858178859639205706, 8791066761743017347, 990442083123263665]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7168100213492872477,6331892997364474120, 7979179634712411422, 156093538247180853])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3257286673034536796,1072784333520920, 3690799403435097850, 1975693771566327878]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2922775461666733736,4274164962974248901, 13795212037211279379, 658688595148724519])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9388159628128294222,7535642247259862856, 2542528849068039859, 455662742064629315]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_48() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4322017323759760606,17980087147840381388, 5894344931218393771, 2544190590391296969])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16745521794377650508,17514057937962400360, 1036372208798824225, 551586093463465010]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6735814093237462131,2480465007479667301, 16188151131296493856, 3461991074047363908])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9898954940426327280,6076495017462115232, 4150480906834010220, 1419172859367567567]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17948245506271794686,11690714499917669626, 15642832318421801269, 289356733341029123])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2145641030511246286,2425617884756173611, 14532477675273775440, 3343547627499635968]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_49() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14025781002345227962,8672244001291372019, 3929848129254868040, 2536708383418694717])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6346678467868851296,3149445576011325437, 3057768383496802133, 1426924847909574634]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17425722815126763798,13144679081035219139, 12953218157291123780, 2464051605440467195])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8270515577585088633,8762375861393899946, 5298937365160946812, 2906795730872178055]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([548111515923927359,12484735509647945880, 13404630865866925940, 1656359775583916482])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3697783271195111082,3863979605348793263, 10166594019823997998, 3381426175307973448]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_50() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10207865224797636347,4072978175706144695, 6020749503638002115, 2216400393316191435])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16749458423201136149,18186530199725594177, 5414319660031269942, 882753875468356262]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5964294957361152255,9368302322794278783, 14886860529502561794, 1379227126573986046])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10367585295218560959,17357747402997226228, 2371601428805115040, 1363762000784729549]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11078909124980082463,962080722592534416, 17063529890710603687, 3101932821364311345])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8098821170691713809,7404878572698358306, 13944888070228047853, 1777895861068085800]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_51() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2055510446142504911,13012402624022420402, 2973334958074097582, 1080597336934029242])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3496507731046048740,11336774843619233275, 12303481569062014142, 2708818508181536283]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([998449473484138077,8826190016492823746, 3138485020977764800, 3114209280254491466])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([167446970588208633,996956914312872247, 11208504658320112001, 3307227784659703392]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1122874478049871063,10463323623080159486, 2731495695691084094, 2858189561279495434])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15522086614313253203,11873471704314546005, 3384026840830554484, 1802771619145507879]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_52() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9534676487630005589,17061480296897207751, 16860614668993282335, 787681848685494618])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2485246747181579837,9285928141069710041, 6617094370507669312, 1892016548511493390]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2143179340947583129,16294108482535268249, 13966206387312111393, 2160666917657776806])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14578674705706265618,762808421175672092, 4854682868286180026, 2450897586603143033]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13533384312710723300,17069109599013035075, 15749327797893465546, 2459470325273210370])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8681516137523667606,12491657029140253437, 14583380684057178261, 1977379331361138016]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_53() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13360589520090960334,17453175483423346329, 2639766512464487513, 307701385541779826])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17219951413093152470,912602394943151325, 12732802899293597780, 426267589092451139]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5551966560703035582,10933794268441517933, 9194418337787473423, 100357203564063629])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([43787914095096163,5477039635038752456, 10314786024549262397, 2673543704487345687]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2507015030478322179,13547114875866966923, 13669206072426306467, 3038403379207739789])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7124391793219668097,14323419881874587110, 17294218475574023465, 3280616206646865604]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_54() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3449032672058215346,4823421583130444775, 12921711791488714917, 470060235924309276])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13533012567031696053,10727262117884187443, 3624594539100424678, 3453814295881321884]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1592070834364788624,14230463392562157846, 16957978382290308582, 1442854378429784120])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15686331637508625014,2356728277271280575, 7958329061238940366, 2844605968416204101]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14475672975429103293,10450618779442713107, 4292707624464510937, 668328951767303082])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13675863161392412827,5274399002195036479, 1532443978643260791, 3095042581999078691]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_55() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17113792854210030655,11474586805090220085, 2114791923243180541, 2392301504563090463])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3885972612353942646,16128835032427240378, 17991128984220371733, 1755075459113658047]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1733172762237453468,3136169898559511156, 5476487739273701259, 437443457999319603])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4703218331996146575,11843142422876626333, 9873520732906582393, 404373496565417972]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10308923232336672100,10073735899149430211, 15402724523929543215, 2179336206609819640])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9878892366814221711,8111983330133954930, 5583594964260387461, 314176048089098313]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_56() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([282927506729308192,6013210819763094055, 3409446690260020528, 3238190068307313887])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([661513209568797844,1982012433731178003, 11783573488610984385, 1483831657558205149]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([698638771842894364,9118823954964398361, 5099293708605589600, 493664512496660171])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17401321793896610972,3043392787228622465, 17622967951418612679, 2209245377349010156]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13892908576542684728,2492835021233125832, 17579455987479734469, 2498438353879322976])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1962897471194263,16912898885226247319, 16345138748486255247, 2084673501206417132]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_57() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([465444359355924577,9604637363538011909, 7784168754973083548, 1760980813273401362])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12305656074270282282,11571320522456263768, 6935657603241083006, 2073467395574762704]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11981921176456049209,5950769990423317802, 17471316374826740817, 2499681299168993493])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5714797801065677276,11737179442024326631, 12661291457663840107, 2848500412446443601]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([683718265923967662,10850120882158916412, 16089543959640990195, 350612157704584674])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12837290076070221515,17457713333555408210, 6009729706902179235, 457329597151258411]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_58() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11828943094652745226,7515803048529860699, 944598861862780971, 2256125560713741129])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9436903937984850409,10554664947127813265, 16195984534370814768, 3412949939276694889]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4803243753965677280,15220221380120731144, 14365151364897903785, 3177616898645270705])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15174317696770310475,17679469948807319964, 3081702695809419068, 3333028656322991759]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8861148386478841,4296258791554993340, 572846923848837612, 3102289652638487166])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1415958468360552970,9073614507012889015, 8239101862710745015, 949560227564696379]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_59() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([779567123320378422,9463429410404172061, 10218098951237933731, 2386392997248357346])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11521393638536892123,16993149599774343754, 3410086311639585031, 2059841237907185843]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7894038386306930099,2678696793399092699, 9003899495056256922, 1139389689448920248])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1854555032756406057,4189741863040754180, 7521907038738677190, 2607856370335628007]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7616920102500080443,8430868258757964903, 13202882461398737425, 2353188662270702756])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8807432936119894571,14139472274313377236, 11914049333314575436, 3476300374405075626]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_60() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4146819762964418085,12813770949761042353, 11264838815583241685, 2462309478638357196])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11844854683201272204,4856169682077447106, 2571292852418849828, 588759663189135476]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6159669794455829648,3485357303284074392, 12358742917947417587, 366228441329263595])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10903622241815791000,5926922369772572546, 13808279783322091648, 2272581476997126038]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6542623947080543251,9192527458535999548, 8842146912958827467, 3254659942544195574])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1271604099219239646,6434215463853483331, 9226336183452747077, 2027650683532935498]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_61() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1242101660225467705,6277106277092667129, 14812272633388065801, 3252528407604977585])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([673203496934061581,17106709749465168979, 10881402657441439329, 1621776420307535627]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11931253009717631587,6820969212233525890, 12712390633612418938, 3038303250264449740])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16697420824076445166,3350006209337524573, 4313959626247587812, 1731086892560578082]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12421993951378968809,4791225357173105696, 14719946857770761136, 2919492766031920926])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5810729306929505165,4257309947703124379, 12932298279375058626, 1172359490694743819]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_62() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11183221920551798483,5545161499101469286, 4940866138184461740, 246755323974120486])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18307538122556030043,4564028050053288764, 9508269527846700054, 1544121138031735361]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11175798691182381955,4589157003932524654, 6525706557774674789, 1996697530976069058])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17877772987999253547,4753079190641503917, 12347850320042286899, 1370525390464709869]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14771160207530338836,11543812155299088250, 16102309284866036249, 1573393854995361649])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3531122142760416358,16480603259955162010, 10202883322015563815, 2217988432857053833]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_63() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1074284945181680297,12620860439386017047, 4716433615500309917, 62043245160064782])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10400560218807955759,10395764944890485459, 13171933842021566440, 2100599025663036809]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3286696714746032125,9249543742977563270, 12811985817933299573, 741949945591202401])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13432642289073689141,4248772011502971515, 14296245104294109878, 1414272386677440180]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13298332506894318619,776381846311218800, 255387634309260536, 842282337771811532])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7399475434846698444,5555357550068793523, 9524198843923658042, 2343439486243618214]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_64() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13966409008759395319,11033205768042455637, 13410159500039578109, 3412719855885316194])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14611491441758721771,4515741406743084136, 17585649370799263192, 1432894327345858827]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3983126890470562184,1931975151886313113, 16008707163410195003, 2946762942368632153])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1462354122943661827,15996484331844727189, 16779116812292739070, 847722284493678586]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1584397420460269267,17508535665004964446, 8185719024897533101, 1048221153854249413])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16245081751391085916,10131844433966115153, 4632734018354412098, 2494174964963854956]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_65() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15156708208799676533,5603177861049236163, 17721123825318688665, 1626875404085262030])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16272998976172085071,7089894560100535206, 11502016064942282852, 2729034002735557919]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4661239578830491908,11683454068756369754, 16715265910289019012, 966900860652284775])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17991495113920867435,17468934527238891509, 5921716021300824825, 2405927716379054257]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15228607643580998669,1020768590662996428, 13224665303113297702, 3324121100235084529])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8021576204764400221,17245004691905418321, 10093346342588594774, 1318540007075835961]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_66() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1104347389170469443,12190259935820161666, 15048740275763655886, 2297424345704808634])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16290705356169112809,12436552693223461975, 8602424628741196594, 3005678111189292324]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15973867031951113215,9899953534808993272, 13438684467853537111, 751097433202024574])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14718554376691713479,10010648790168896337, 6731489145279266963, 413309367941568960]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17526557060103281794,729632802214743053, 10825728610539299728, 1839769218049176184])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17901319544077037644,13277325228260942631, 11441413736762331704, 102848413302941560]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_67() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([2631964006125485935,14561461056510935405, 4491976214861129469, 2414586922851620425])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14002907804468300704,9737460847783161530, 14842180658160999493, 901692265503574117]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14773640291318286661,6676399762824962280, 1746152515032157829, 284300670880268577])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1060593575350836775,13563055424500951815, 7830875817341074820, 2141727797425378503]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9158490205696524437,8058790016054695130, 2915063926416851201, 2421623830567540740])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4589019104482819263,16402183971096794815, 9638251072414720254, 2206028251318410342]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_68() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5688294628041221146,11168983897981852655, 13818670592921920485, 1379238968913602559])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6877892942126654358,9425041407390249478, 15068563527499974594, 295672602949272126]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12551234083934265827,2079807671436094666, 11838286205458842679, 2526723458483867411])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10876486575143931911,14113198036068690714, 5163810108839532639, 2142214221293540105]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3440703304870576998,4703220341127719686, 12536711160089449940, 567727122633803068])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([377077773692141263,17726977844225873404, 4896399496060769497, 2427728326197743504]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_69() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4995191565605330045,359927033101112803, 8705005819660256978, 1210061418088952891])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17150929980495404022,15236666448989099377, 13143024241015495329, 1479306764843151575]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5682081168609789189,14432695214647731737, 17986586628620276785, 2494121032441421830])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13591258608840022876,11018488180302515622, 2271693950243260403, 1552155104112619019]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15861207985539730389,6879332390279103038, 12974724538517376264, 436474107668194343])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16142891022573853793,4750516243950462286, 7687589319820657597, 2773052573771566468]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_70() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4244092801103956951,9930283473133888802, 6280607451131672606, 1916463594839298330])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4152972457388096824,17010990070431603851, 14823565631265723985, 2691608202341410823]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8988113908061507410,10264916581926325865, 17287541281285988333, 531034811481039861])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4952828782857829040,13306932053655640518, 13997729969392011718, 510394875075305934]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11196305487427580362,7722561156796175970, 12537468805604946486, 2938265308161930874])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3321093485834050413,3603617962919432357, 8601158408812128720, 2043297068256605335]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_71() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3729055922388708693,16355018326140358782, 12837377597103173339, 699234350489456403])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9713610174436412008,8311579558668705054, 9292974672988964669, 942728393652219598]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8542796165071350323,8308198745174982858, 9170743146778675913, 2420346915878929469])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1471743986074370380,5197534366130505376, 5221771671668313590, 2206744011335745890]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5456490630250243872,17834992803144306652, 8669007398573749046, 738425436479172309])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15937268125811235940,6458198169650460462, 9836688358121677916, 860326569802781648]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_72() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14786338373612424940,8001917640461740221, 14042665108151070730, 3080468296109089940])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([51962898343389655,5261682212063803478, 11926581049015371598, 203523292631563009]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3301213481545835435,1259603581128175336, 17328191437235601609, 3032092733487086958])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3380109742653375923,7703927429450619936, 708213448892164054, 342407091295040470]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12413037917533076379,13701517961897152440, 10798565187099965715, 1941539806909564630])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10232343776795782427,16387224896086041006, 923291106702340202, 741081742436410687]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_73() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9068550242696846838,2029734138902951023, 7921470686268239842, 1582497552715005306])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15229089632956481605,17070433446593512121, 1296413663070854313, 3349181049667681887]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11009057667770568519,16043255198752984730, 13634965281041569942, 1514721912915771076])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7349412446992907682,8842267720099781712, 10463182018001147902, 2673434799582324070]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11156755860864290755,1972753770568625282, 2019760522634820214, 69060144076587758])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13087409746466965400,5420333572312213037, 7166173900087895212, 1684975756755737389]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_74() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13334502209498727352,11502826491701306642, 8839239913377287922, 1335179580829788772])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10693065976249287006,8741517679667871504, 12242740908457318897, 682104718502235304]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8164963155813341874,5097601794398525934, 2352073265476348196, 369673596113636616])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8182101570212895902,13142207854301675802, 6000629895292768071, 1864820448037713366]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3921578436255774121,11014171138184295778, 16946489461258811080, 1140457727565506739])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6020719129756185155,1470995783551129043, 14143524315796593290, 2024181305439075408]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_75() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6762565154686783613,16766800687095770276, 2679256012252553570, 818222117140247625])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5008834269160458890,17076787203210669579, 6381129229798636577, 1630450639115874878]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14628678762703488750,3062341996632895238, 5350492541751644647, 1950431321474690959])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([728461585386147405,820475403855487459, 14870519945927433216, 2941829677143960676]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13615501404914910828,2518505870851574346, 13101245291651166131, 588156335952268090])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4897375444114482065,18116978395500505048, 2841575515022238278, 1553700144612712530]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_76() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18262717665743340178,2527993262527088940, 17718027719464381732, 576551145218726064])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1277875848967198926,2286602466566189204, 3953105079212223547, 901313723567368950]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17835129696127142002,16011671044211194886, 8933166668334336223, 1090695052223311000])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8044144578999977452,8695244951325647909, 7550516894744968444, 651481343762461345]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([12553118163203338789,15226210722212147126, 2555089678467222255, 2644652830073287691])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13264233842493855546,3125342471681383491, 16264415023442989896, 2716993185462949442]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_77() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([847193155799115914,14285688453571936707, 952247975259691664, 3225922342019492604])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10042904037614070011,6223867915675830872, 7910638145045231997, 1286587112939306067]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10348973456400226718,9255030028136196973, 17416789687649482724, 1622045568848995304])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9428984264688733524,5635141077921423290, 18424392876405044747, 1203776544378865883]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10686937349477332926,6338531311963652381, 4545786465254207935, 2333008442711493107])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4009375928213911815,2289255147419256639, 6998443708214427067, 2363205372861110681]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_78() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10296408567131672711,18289570584988902554, 10023511322837383889, 2846897256622173379])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5969957899280269568,17188075476724464390, 4426640271717385987, 483769465854293240]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1595571705123842119,8725479654035472050, 6809282319989669149, 277857815869873708])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5538718896707520790,17585460901843804740, 1834540071493114166, 1455260892430689680]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9177458921462054563,696676440248411697, 9061447510272587809, 2439551720804971704])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10038865381658183974,15496845275351251165, 1749146613361318756, 3243562956794692407]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_79() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1092339859164823662,2349559426228506177, 4293682831954645727, 2603795516193414574])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9965880023355273199,1358531242899204818, 12336688098291915209, 1455546316026579508]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([10923258099365973550,3602270364430049369, 11201362581428432631, 354802400255665840])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14602776593052295879,7379422964718711913, 15806315045593380048, 852228050666443891]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15655876100356932583,3547742095979405751, 4593917165459460683, 3052404992522257316])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([611158024298224422,1755915009658253318, 17335601270903645812, 1065776225169509563]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_80() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14198795184326759869,12540708723650205023, 4627052965007222834, 2698670066067178164])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9037672868730975302,10222360970034863686, 2441912910922126844, 1354142276992796973]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5041774744472109493,4409457498339537166, 13073858100384820531, 5240643098523833])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14179690518258323993,2230065348099296522, 11482850388284240022, 1732252615507563020]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18306379752796073239,7032165670033413415, 6761003356140898196, 3270888446853318398])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8476059711433158341,16676674472120157206, 5755200367279255314, 2825906104670860344]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_81() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9015092980327714731,12945185432538819372, 10855647611189663713, 1347288474180126986])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6177681001884700435,4090943223766189346, 14871413201648300074, 2521599690676181029]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9499451662448931317,11922591927375937318, 2636661872769342017, 1410067357059596102])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([654424215647297743,11803485517719226819, 12610493692240273470, 1518590503512987495]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17919017195456192822,14844270302592977437, 17220736319792215085, 1311100388563630741])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7963304786122302914,15970316635596421567, 15796426704715714178, 619437263137175210]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_82() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4724654422440308118,4017660140895120077, 16098862807750544196, 2104704179589058088])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5930648235210481752,6992124989519641478, 3585453955039832697, 1250377137446867144]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14543658446720882086,646917583586304627, 10206018280829923252, 1035468055746689093])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11583808385117200458,2675385617832129896, 9227325068151918817, 1108889213166721563]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11274637040474956286,5856535405070266360, 8622828182267716518, 1463865908062533327])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15013428224535760134,9819657003615896353, 7985742866326827328, 863404631444462528]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_83() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6943141645018346692,16170116613213145258, 6829059600315438553, 3422762892409688297])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14255468155326044669,17147983455793698827, 14266527996098756112, 628661935350266655]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17497195789908846022,2424450488545332438, 12011063112764695287, 3331434991355455060])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4206328039531486512,15142969396772168490, 13218030602535437771, 2422719084669521465]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([17198432584708006162,1606763998110551720, 401288740093235444, 1308529355408633702])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9631560700536347897,7278315375823268302, 1722113823967781381, 3054003186364493771]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_84() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1009397045182096610,3213159481085869476, 4775193086410381386, 1013622881380778384])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([3029404629236760881,5073071344420619064, 4878386805639745438, 1629748134611874723]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7556169466936259185,12729769339851186833, 10236256607980784648, 2541083295861985132])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13382623612442345803,7337881702254920401, 17042889721484091018, 3425211158891417996]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7519940363145399971,16657521135760336049, 16050491895562458567, 73646369248207670])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8838465723659267347,12548944116849494799, 18345281171488624307, 2652679897206858678]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_85() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1632073801589631378,17555434212327352471, 3042010489706345861, 3276636231584423413])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15655646280020594199,13276086752344025650, 13705512640830170554, 3116649070193353780]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13871687808276273413,16906487163040242006, 11778692344308376647, 453327826065626771])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([1344415988838927283,16006212086933404078, 11634136515618926094, 2596720669615034986]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14394468634967399306,2374195568848656784, 3735565975193327563, 2455083214679435025])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14504826400039323241,9624811584916696791, 1831295318804374733, 681334297557853068]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_86() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7284357993242295593,17024249923404251955, 15770068472956457276, 3297225945691602247])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14639328436404327764,7020314158362973191, 3461133781630627292, 1526891798729612355]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7030143060581488513,14444475415022260634, 7587528409311986941, 1560308413735444621])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([4126500381090282032,3300709085854840757, 7461357259277696580, 1322612020341570412]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([5032944572972437500,4596735871744272925, 1338082986364222909, 2300713668272981230])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13978517626290576887,17808848879656933941, 7638312982719631438, 1995430409333197224]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_87() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16580056233860023652,13137181868257732996, 11063676101962075677, 2992566619077348355])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15709208113782291470,3696262484203399115, 13814231783742736358, 912972915516657468]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8844706418218486970,13171543236409155471, 5367992152976678788, 1208897546404348417])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11264508775652973227,12287599599472355384, 926573881248569998, 1404001205654525579]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([16969400582414337722,4645579231102465269, 1259129474187460044, 111034341868306713])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15881772249393847648,11811212124864557383, 9157182464268642569, 2833073768953710605]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_88() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6470576453444509268,17877157367631513323, 7330310891509505255, 3168891646173001530])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([9746290069860533343,7352718346178923409, 11730671178959503549, 2279963326424867370]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11255475388158715855,9145815210684968703, 9177383493392115046, 396037402603282771])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7324173248770832686,8924834313475757869, 3619400909721314516, 1737864266812239879]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14022369366412431351,17163577299523550947, 15991981205545037090, 2694689751206598628])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([15364778945545811418,5999217701820118386, 17764022673108059072, 212733006351430019]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_89() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6474845769800915397,12099393162132130645, 6061460649073770242, 2955768612019259913])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([6076026172079522793,14541586695526824955, 14057600450587858250, 1522119216819395291]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7786157735423021104,16549642331696274843, 1805734605458999555, 640755370832653090])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([11277711616401804821,2253393432885364107, 7072483629939774567, 20903688069974813])) 
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([274036021740537553,1610792986149775775, 4175764721418308033, 598965822833793321])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([14308601657228676082,1465458979053425130, 8901653056760640539, 1899352279081457410]))
 		)
 	)
}

pub fn get_delta_g2_neg_pc_90() -> (QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>, QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>) {
 	(QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7193852487383098550,10397210697689998874, 769690836599231366, 1116553396124340870])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([7425523609391266569,8615857865985054815, 15727025352582461160, 1536428110351553128]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([13122635266683088171,9356371006440968690, 1919014766096075213, 1122886154097170069])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([18390927234086458936,8650586290514038094, 10155263784405445013, 2402823687109097389]))
		),
		QuadExtField::<ark_ff::Fp2ParamsWrapper::<ark_bn254::Fq2Parameters>>::new(
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([481089311513715273,10037468427058564492, 269132250396066765, 167701434583626259])),
			ark_ff::Fp256::<ark_bn254::FqParameters>::new(BigInteger256::new([8727435674745872713,12997230226130352674, 15254863982842491616, 483193518599528182]))
 		)
 	)
}
