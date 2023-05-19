pub mod data_structures;
pub mod error;
pub mod prover;
pub mod rng;
pub mod verifier;

use ark_bls12_381::{Bls12_381, Fr as F};
use ark_ff::{Zero, BigInteger256};
use ark_poly::{
    Polynomial, univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain,UVPolynomial,
};
use ark_poly_commit::{marlin_pc::MarlinKZG10, LabeledPolynomial, PolynomialCommitment};
use ark_std::{rand::rngs::StdRng, test_rng};
use blake2::Blake2s;
use prompt::{puzzle};
use rand_chacha::ChaChaRng;
use rng::SimpleHashFiatShamirRng;
mod flag_check;
use crate::{data_structures::Statement, prover::prove, verifier::verify,flag_check::{woe_jinx_death,flag_quest}};
use std::{thread, time};

pub const PROTOCOL_NAME: &'static [u8] = b"OPERATION_ZK_RESCUE";
pub type PC = MarlinKZG10<Bls12_381, DensePolynomial<F>>;
type FS = SimpleHashFiatShamirRng<Blake2s, ChaChaRng>;

fn decode(num: &F) -> String {
    let binding = num.to_string();
    let num_str = binding.strip_prefix("Fp256 \"(").unwrap().strip_suffix(")\"").unwrap().to_string();
    (0..num_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&num_str[i..i + 2], 16).unwrap_or_default() as char)
            .collect()
}

fn main() {
    puzzle(PUZZLE_DESCRIPTION);

    let domain_size = 32;
    let domain= GeneralEvaluationDomain::<F>::new(domain_size).unwrap();
    let max_degree = 32;

    // for p in domain.elements() {
    //     println!("{}", p);
    // }

    let mut rng = test_rng();
    let srs = PC::setup(max_degree, None, &mut rng).unwrap();

    let (ck, vk) = PC::trim(&srs, max_degree, 1, Some(&[domain_size - 2])).unwrap();

    let poly_evals = 
    vec![F::new(BigInteger256([1121870363005239449, 9833063431898617369, 11114025489853872418, 4089459679630506955])), 
    F::new(BigInteger256([18389056416795951153, 12776612665753701770, 10601712950505962344, 7802313184811879854])), 
    F::new(BigInteger256([16067594266876709090, 8470427502427084384, 16443615647861417996, 4625471000544485246])), 
    F::new(BigInteger256([5410017001786458903, 11675705801598402565, 6498911959740880938, 7239757810459545720])), 
    F::new(BigInteger256([16329743844246186018, 7338568047734119830, 4888220250742468692, 6585733539041573621])), 
    F::new(BigInteger256([229393851038631626, 6997576012805149166, 1786443055583271237, 8106042075235365182])), 
    F::new(BigInteger256([17888586599545895052, 5980900867706673195, 12596916618496819229, 3222949190399674667])), 
    F::new(BigInteger256([5886651830583060648, 225677719457626121, 12983813538641759408, 3606877985381565199])), 
    F::new(BigInteger256([9249640068201374999, 6966605152766553332, 9800649042352009920, 5046532559012371444])), 
    F::new(BigInteger256([13132415283921178503, 9307134057080303314, 900958139024845688, 7933636746154525278])), 
    F::new(BigInteger256([11497368629690836005, 5390006342406776795, 16088111248549898431, 1050413031973697574])), 
    F::new(BigInteger256([16092568367837947407, 16801031190279933494, 4126268621267045913, 4898442335951448540])), 
    F::new(BigInteger256([8934298324614403177, 9116335957124190768, 10219872942150197737, 1841166463301124616])), 
    F::new(BigInteger256([11984246573213425040, 3960674218739690524, 10793507688983291465, 4232054263828424158])), 
    F::new(BigInteger256([10397903606860953882, 13183409075177529230, 4822801348076544491, 5016500350000436620])), 
    F::new(BigInteger256([10014794955098442830, 13888390611927284848, 11856051024870441770, 8279537559651985880])), 
    F::new(BigInteger256([8230672736884221707, 18006485274867283109, 1629228829033113634, 1099330250190027198])), 
    F::new(BigInteger256([11015006238666753150, 9364099738537891123, 6173991011148012520, 3781048326175269637])), 
    F::new(BigInteger256([16006716131481113535, 626378568218358891, 11419691545895483840, 850040481719240847])), 
    F::new(BigInteger256([12460826533802688585, 14648978760728443831, 15244380280428997107, 671881350330993016])), 
    F::new(BigInteger256([10855893703435144320, 8804277236749862193, 14601128575889677579, 4462652008705522343])), 
    F::new(BigInteger256([10294062090920666226, 6482153579859727405, 15735045722273410134, 5043545826792351954])), 
    F::new(BigInteger256([7111366436266622085, 10959633784297204957, 15923749305820098937, 1114792912864000057])), 
    F::new(BigInteger256([9537722138432320427, 10895844386941714665, 10104125960919209227, 1034866158661188961])), 
    F::new(BigInteger256([6686650250179095170, 18336973546939458050, 187096290107898103, 5592607494254475267])), 
    F::new(BigInteger256([2995922417646065600, 17679839272067967215, 5762804870460140671, 3869670751169999110])), 
    F::new(BigInteger256([14386549208691603756, 8234036546237469535, 6823157361518105112, 2382734757411032088])), 
    F::new(BigInteger256([15844849751539503014, 9932767315274314249, 6123114619200601223, 2082289421510119762])), 
    F::new(BigInteger256([5690401103993610694, 11013490713874630348, 17189187543924385957, 5595667519177664099])), 
    F::new(BigInteger256([163440685443930, 1168097028314557606, 6312152460365060130, 8010052176547349905])), 
    F::new(BigInteger256([11918697147346910008, 9233808242802061551, 14395435043740768310, 4872975318810306762])), 
    F::new(BigInteger256([12098040391674830792, 13279254240913572044, 17820649906232228079, 6653899853934111562]))];

    for i in 0..domain_size {
        println!("{}", decode(&poly_evals[i]));
    }
    println!();

    let eval_sum = poly_evals[..domain_size].iter().sum();
    println!("Flag: {}\n", decode(&eval_sum));

    println!("Sumcheck preparation..\n");
    thread::sleep(time::Duration::from_secs(2));
    //interpolate to coeff form using ifft 
    println!("Encoding identities..\n");
    thread::sleep(time::Duration::from_secs(2));
    let poly_coeffs = GeneralEvaluationDomain::ifft(&domain,&poly_evals);
    let f = DensePolynomial::from_coefficients_slice(&poly_coeffs);

    println!("Flag: {}\n", decode(&f.clone().evaluate_over_domain(domain).evals.into_iter().sum::<F>()));

    let sum = F::zero();

    let f = LabeledPolynomial::new("f".into(), f.clone(), None, Some(1));
    let (f_commitment, f_rand) = PC::commit(&ck, &[f.clone()], Some(&mut rng)).unwrap();
    println!("Preparing the statement:\n");
    thread::sleep(time::Duration::from_secs(2));
    let statement = Statement {
        domain,
        f: f_commitment[0].commitment().clone(),
        sum,
    };
    println!("Beep! Beep! Incoming transmission: \n");
    thread::sleep(time::Duration::from_secs(2));
    puzzle(FORGER);
    
    let proof = prove::<F, PC, FS, StdRng>(&ck, &statement, &f, &f_rand[0], &mut rng).unwrap();

    println!("Verifying the proof");
    let res = verify::<F, PC, FS, StdRng>(&vk, &statement, &proof, &mut rng);
    thread::sleep(time::Duration::from_secs(2));

    if res.is_ok() {
        println!("Sumcheck validation successful!");
        thread::sleep(time::Duration::from_secs(4));
        flag_quest();
    } else {
        println!("Sumcheck validation failure!");
        thread::sleep(time::Duration::from_secs(4));
        woe_jinx_death();
    };
}

const PUZZLE_DESCRIPTION: &str = "\
Agent Zulu has gone M.I.A.

We have received reports that he has been kidnapped by the notorious Woe Jinx. 
Direct intervention without evidence is not an option. 
Our friendly enemy The Concierge of crime: Red, has managed to get one of his associates 
(The forger) infiltrate into Jinx's organization, who will be your point contact.   

Your task is to send us a confirmation message that indeed Zulu is inside Jinx's base so we
can rescue our man quietly.

Jinx uses a sumcheck protocol that validates the sender's identity in the base,
when the sumcheck evaluates to zero.

The Forger has forged an identity for you in order to faciliate a one time message.
However, we ran some tests and found that it may not pass the validation.
We have no idea what game Red and the forger are playing here. 

We do know that Woe Jinx protects his men from HQ by anonymizing the validation process, this basically
adds a random polynomial to the claimed polynomial. This is usually a real pain in the butt. 
But, perhaps the anonymization can be used to your advantage this time. Just watch out that Jinx double checks the anonymization, 
so if you use a constant polynomial for anonymization, you will get caught!

Once you have cleared the validation, we will use a security lapse window to activate recieving a one time message from you.
We have been told by Red that you will have to eventually find some of the information you need on your own. 
U have got Big intELLIGENCE, be YOURSELF! We are expecting your message in 8 in the futURE. 

Note that if Jinx learns the message during the validation, the probability you will live is pretty low.

One more thing, Red and the Forger cannot be trusted, there is always more to what meets the eye!
Watch out!  Good luck!! - HQ
";

const FORGER: &str = "\
Hi there this is the Forger, I have crafted a identity for you and it is in the form of a polynomial. 
Note that you cant change the given polynomial. You have to use your knowledge of the
univariate sumcheck protocol to complete the code in the prover.rs file. 
You may want to consult the attached document in the docs folder for reference. 

BTW: Red said 'You will have to find some of the information yourself', you know what it sounds like right? sigh..Goodluck and Godspeed.
";
