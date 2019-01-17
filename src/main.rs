use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  // day1();
  day2b();
}

fn max(ints: &Vec<i32>) -> i32 {
  let mut max = std::i32::MIN;
  for i in ints {
    if i > &max {
      max = *i
    }
  }
  max
}

fn min(ints: &Vec<i32>) -> i32 {
  let mut min = std::i32::MAX;
  for i in ints {
    if i < &min {
      min = *i
    }
  }
  min
}

fn sum_ints(ints: &Vec<i32>) -> i32 {
  let mut sum = 0;
  for i in ints {
    sum += i;
  }
  sum
}

fn find_divisibles(ints: &Vec<i32>) -> (i32, i32) {
  fn is_divisible(a: i32, b: i32) -> bool {
    (a / b) as f32 == (a as f32) / (b as f32)
  }
  for i in ints {
    for j in ints {
      if j >= i {
        continue;
      }
      if is_divisible(*i, *j) {
        return (*i, *j);
      }
    }
  }
  unreachable!("Should not get here")
}

fn day2b() {
  // let input: Vec<&str> = vec!["5\t9\t2\t8", "9\t4\t7\t3", "3\t8\t6\t5"];
  let mut input: Vec<String> = vec![];

  let file = File::open("data/input2.txt").unwrap();
  for line in BufReader::new(file).lines() {
    input.push(line.unwrap());
  }

  fn to_ints(str: &str) -> Vec<i32> {
    str
      .split('\t')
      .map(|v| {
        v.trim()
          .parse()
          .expect(&format!("Could not parse {:?}", v)[..])
      })
      .collect()
  }

  let input = input.iter().map(|line| to_ints(line));
  let mut sum = 0;
  for ints in input {
    let (big, small) = find_divisibles(&ints);
    sum += big / small;
    println!("ints {} {}", big, small);
  }
  println!("sum {}", sum);
}

fn day2() {
  // let input: Vec<&str> = ["5 1 9 5", "7 5 3", "2 4 6 8"].to_vec();
  let mut input: Vec<String> = vec![];

  let file = File::open("data/input2.txt").unwrap();
  for line in BufReader::new(file).lines() {
    input.push(line.unwrap());
  }

  fn to_ints(str: &str) -> Vec<i32> {
    str
      .split('\t')
      .map(|v| {
        v.trim()
          .parse()
          .expect(&format!("Could not parse {:?}", v)[..])
      })
      .collect()
  }
  fn checksum_ints(ints: &Vec<i32>) -> i32 {
    max(ints) - min(ints)
  }

  // println!("initial input {:?}", input);
  let input = input.iter().map(|line| to_ints(line));
  // println!("input as ints {:?}", input);
  let input = input.map(|ints| checksum_ints(&ints));
  // println!("input as checksums {:?}", input);
  let sum = sum_ints(&input.collect());
  println!("sum {}", sum);
}

fn day1() {
  let input = String::from("3294199471327195994824832197564859876682638188889768298894243832665654681412886862234525991553276578641265589959178414218389329361496673991614673626344552179413995562266818138372393213966143124914469397692587251112663217862879233226763533911128893354536353213847122251463857894159819828724827969576432191847787772732881266875469721189331882228146576832921314638221317393256471998598117289632684663355273845983933845721713497811766995367795857965222183668765517454263354111134841334631345111596131682726196574763165187889337599583345634413436165539744188866156771585647718555182529936669683581662398618765391487164715724849894563314426959348119286955144439452731762666568741612153254469131724137699832984728937865956711925592628456617133695259554548719328229938621332325125972547181236812263887375866231118312954369432937359357266467383318326239572877314765121844831126178173988799765218913178825966268816476559792947359956859989228917136267178571776316345292573489873792149646548747995389669692188457724414468727192819919448275922166321158141365237545222633688372891451842434458527698774342111482498999383831492577615154591278719656798277377363284379468757998373193231795767644654155432692988651312845433511879457921638934877557575241394363721667237778962455961493559848522582413748218971212486373232795878362964873855994697149692824917183375545192119453587398199912564474614219929345185468661129966379693813498542474732198176496694746111576925715493967296487258237854152382365579876894391815759815373319159213475555251488754279888245492373595471189191353244684697662848376529881512529221627313527441221459672786923145165989611223372241149929436247374818467481641931872972582295425936998535194423916544367799522276914445231582272368388831834437562752119325286474352863554693373718848649568451797751926315617575295381964426843625282819524747119726872193569785611959896776143539915299968276374712996485367853494734376257511273443736433464496287219615697341973131715166768916149828396454638596713572963686159214116763");
  let input: Vec<u32> = input.chars().map(|v| v.to_digit(10).unwrap()).collect();
  let mut sum = 0;
  let len = input.len();
  let increment = 1; // <-- part 1
                     // let increment = len / 2; // <-- part 2
  for i in 0..len {
    if input[i] == input[(i + increment) % len] {
      sum += input[i];
    }
  }
  println!("sum: {}", sum);
}
