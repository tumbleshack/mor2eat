use std::error::Error;
use std::fs;
use futures::executor::block_on;

mod locator;
mod graph_builder;
mod utils;

pub fn run(arg: Arg) -> Result<(), Box<dyn Error>> {

    match arg.action {
        Action::Help => {
            return Ok(())
        },
        Action::DownloadCfaData => {
            let codes = zip_codes_in(arg.dir);
            let data = block_on(locator::dispatcher(codes?))?;
            locator::output_cfa_data_to(arg.output_path.to_string(), data)?;
        },
        Action::DecideConnections => {
            let profiles = locator::intput_cfa_data_from(arg.dir)?;
            let filtered_profiles = utils::filter_profiles(&profiles);
            let connections = graph_builder::decide_connections_from(filtered_profiles)?;
            graph_builder::output_valid_connections_from(arg.output_path, connections)?;
        },
        Action::FormGraph => {
            let valid_connections = graph_builder::input_valid_conntions_from(arg.dir);
            let cfa_profiles = locator::intput_cfa_data_from(arg.input_two.unwrap());
            let edges = graph_builder::build_edges(&cfa_profiles?, &valid_connections?)?;
            let _result = utils::output_to(arg.output_path, &edges);
        },
        Action::Test => { 
            println!("Testing...");
            let val = block_on(locator::dispatcher([
                "39817".to_string(),
                "39818".to_string(),
                "39819".to_string(),
                "39823".to_string(),
                "39824".to_string(),
                "39825".to_string(),
                "39826".to_string(),
                "39829".to_string()
                ].to_vec()));
            locator::output_cfa_data_to(arg.output_path.to_string(), val?)?;
        },
        _ => return Ok(())
    }

    Ok(())
}

pub struct Arg {
    pub dir: String,
    pub output_path: String,
    pub action: Action,
    pub input_two: Option<String>,
}

fn zip_codes_in(dir: String) -> Result<Vec<String>, Box<dyn Error>> {
    let files = fs::read_dir(dir)?;

    let codes = files.filter_map(|file| fs::read_to_string(file.ok()?.path()).ok() )
        .collect::<Vec<_>>()
        .join(",\n")
        .split(",\n")
        .map(str::to_string)
        .collect::<Vec<_>>();
    
    Ok(codes)
}

impl Arg {
    pub fn new(args: &[String]) -> Result<Arg, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let action : Action;

        match &args[1][..] {
            "-download_cfa_data" | "-dcd" => action = Action::DownloadCfaData,
            "-decide_connections" | "-dc"  => action = Action::DecideConnections,
            "-form_graph" | "-fg" => action = Action::FormGraph,
            "-run_yen" | "-ry" => action = Action::RunYen,
            "-test" => action = Action::Test,
            _ => { 
                action = Action::Help;
                println!("-------------- Usage -------------");
                println!("    -download_cfa_data, -dcd  <zip_code_dir>");
                println!("          <zip_code_dir> => files each have comma separated zip codes");
                println!("    -decide_connections, -dc  <cfa_metadata_file>");
                println!("          <cfa_metadata_file> => json of CFA metadata");
                println!("    -form_graph, -fg  <cfa_metadata_dir> <connections_dir>");
                println!("    -run_yen, -ry  <cfa_metadata_dir> <gmaps_data_dir>");
            },
        }
        
        let output_path = args[2].clone();
        let dir = args[3].clone();
        let input_two: Option<String>;
        if args.len() > 4 {
            input_two = Some(args[4].clone());
        } else {
            input_two = None;
        }

        Ok(Arg { action, output_path, dir, input_two})
    }
}

pub enum Action {
    DownloadCfaData,
    DecideConnections,
    FormGraph,
    RunYen,
    Help,
    Test,
}
