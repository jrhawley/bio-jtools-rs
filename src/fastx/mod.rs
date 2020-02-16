use fastq::parse_path;
use std::collections::HashSet;
use std::string::String;

pub fn info<'a>(_path: &str)
{
    // values to keep and display
    let mut total: usize = 0;                               // number of records
    let mut instruments: HashSet<String> = HashSet::new();  // sequencing instrument IDs

    // parse the FASTQ
    parse_path(Some(_path), |parser| {
        // function over each record in the FASTQ
        parser.each(|record| {
            // add to total count
            total += 1;
            // convert RefRecord into OwnedRecord to access the Record's fields
            let _orecord = record.to_owned_record();
            // keep the record's ID for string manipulation to extract the machine ID
            let _id = String::from_utf8(_orecord.head).unwrap();
            // parse the instruments machine ID (always at the beginning of a read ID for the new Illumina encodings)
            let _id_vals = _id.split(|c| c == ':' || c == ' ' || c == '.').take(1).collect::<Vec<&str>>()[0].to_string();
            // if a new intrument ID is detected
            if !instruments.contains(&_id_vals) {
                instruments.insert(_id_vals.to_string());
            }
            true
        });

        println!("{} reads", total);
        println!("Instruments: {:?}", instruments);
    }).expect("Error parsing FASTQ. Possible invalid compression.");
}
