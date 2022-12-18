mod java;

use crate::{Runtime};

pub fn base_classes(runtime: &mut Runtime) {
    java::lang::object(runtime);
    java::lang::appendable(runtime);

    java::io::closeable(runtime);
    java::io::outputstream(runtime);
    java::io::filteroutputstream(runtime);
    java::io::printstream(runtime);

    java::lang::system(runtime);
}
