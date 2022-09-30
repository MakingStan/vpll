use std::fs;
use json::JsonValue;

pub fn convert_vpll_to_json(code: String) -> JsonValue
{
    let mut json_value = json::JsonValue::new_object();
    let lines = code.split("\n");

    let config_file_contents = fs::read_to_string("config.json").unwrap();
    let mut config_json = json::parse(config_file_contents.as_str()).unwrap();

    for line in lines
    {
        //Split the line with spaces
        let line_splitted = line.split(" ").collect::<Vec<&str>>();

        if line != "" {
            if !line.starts_with("//")
            {
                let element = line_splitted[0];

                if config_json[element].is_null()
                {
                    //it is null, notify the user of it
                    println!("🟨 WARNING: Element called \"{}\" could not be found in the config file. It has been ignored. 🟨", element);
                }
                else
                {
                    //Get every
                    json_value[line_splitted[0].to_owned()] = r#""width": 100"#.into();
                }
            }
            //else it's a blank line or a comment, so we don't want to interfere with it
        }
    }


    return json_value;
}