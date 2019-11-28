use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ocelotter", about = "A minimal implementation of a JVM")]
pub struct Options {
    #[structopt(short, long)]
    /// class search path of directories and zip/jar files
    pub classpath: Option<String>,

    #[structopt()]
    /// Class name
    pub classname: Vec<String>,
}

impl Options {
    pub fn fq_klass_name(&self) -> String {
        format!("{}.class", self.f_name())
    }

    pub fn f_name(&self) -> String {
        self.classname
            .get(0)
            .expect("Classname should be specified")
            .into()
    }
}
