use structopt::StructOpt;
use toku::*;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long, default_value = "auto")]
    detect: LangDetection,
    #[structopt(long)]
    keep_ponctuation: bool,
    #[structopt(long)]
    default_stopwords: bool,
    #[structopt(long)]
    lowercased: bool,
    #[structopt(long)]
    unicode: bool,
    #[structopt(long, default_value = "Low")]
    precision: Precision,
    text: String,
}

fn main() {
    let opt = Opt::from_args();

    let mut builder = TokenizerBuilder::new();
    builder.lang_detection(opt.detect);
    builder.keep_ponctuation(opt.keep_ponctuation);
    builder.default_stopwords(opt.default_stopwords);
    builder.lowercased(opt.lowercased);
    builder.unicode(opt.unicode);
    builder.precision(opt.precision);

    let tokenizer = builder.build(&opt.text);

    for (type_, token) in tokenizer {
        if type_ == (TokenType::Word { stop_word: false }) {
            println!("token: {:?}", token);
        }
    }
}
