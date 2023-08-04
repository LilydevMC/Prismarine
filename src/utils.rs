
macro_rules! pris_err {
    ($err:expr) => {
        println!("{}{} {}", "Error".red().bold(), ":".bold(), $err)
    }
}

macro_rules! pris_export_dir {
    ($path:expr) => {
        println!("{}{}{} {}", "Exporting".green().bold(), " directory to zip".bold(), ":".bold(), $path.italic())
    }
}

macro_rules! pris_export_file {
    ($path:expr) => {
        println!("{}{}{} {}", "Exporting".green().bold(), " file to zip".bold(), ":".bold(), $path.italic())
    }
}

pub(crate) use {pris_err, pris_export_dir, pris_export_file};
