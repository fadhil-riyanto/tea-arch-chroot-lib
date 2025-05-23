use std::error;
use lazy_regex::regex_captures;
use serde::Serialize;
use duct::cmd;

#[derive(Serialize, std::fmt::Debug)]
#[serde(rename_all="camelCase")]
pub struct Os
{
    pub name: String,
    pub path: String
}

impl Os
{
    pub fn get_other_os() -> Result<Option<Vec<Self>>, Box<dyn error::Error>>
    {
        let mut oses: Vec<Self> = Vec::new();

        let prober = cmd!("os-prober").read()?;

        // For testing purposes
        // let prober = concat!(
        //     "/dev/nvme0n1p1@/EFI/Microsoft/Boot/bootmgfw.efi:Windows Boot Manager:Windows:efi",
        //     "\n",
        //     "/dev/sda1@/EFI/Microsoft/Boot/bootmgfw.efi:Linux Boot Manager:Linux:efi"
        // );

        let entries: Vec<String> = prober
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        for entry in entries
        {
            let result = regex_captures!(r"(\/dev\/[^\@]+)\@[^:]*:([^:]+)", &entry);

            if let Some(result) = result
            {
                let path = result.1;
                let name = result.2;

                oses.push(Os {
                    name: name.to_string(),
                    path: path.to_string()
                });
            }
        }

        if oses.is_empty()
        {
            Ok(None)
        }
        else
        {
            Ok(Some(oses))
        }
    }
}
