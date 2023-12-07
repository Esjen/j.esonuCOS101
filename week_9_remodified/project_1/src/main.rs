use::std::io::Write;

fn main() {

    let lager = ["\tLAGER","\n1.) 33 Export", "\n2.) Desperados","\n3.) Goldberg","\n4.) Gulder","\n5.) Heineken","\n6.) Star\n"];

    let stout = ["\t\tSTOUT","\tLegend","\tTurbo king","\tWilliams","\tblank","\tblank","\tblank"];

    let non_alcoholic = ["\t\tNON-ALCOHOLIC","\t\tMaltina","\tAmstel Malt","\tMalta Gold", "\tFayrouz", "\tblank","\tblank"];

    let mut file = std::fs::File::create("drinks.txt").expect("failed to create file");

    for x in 0..7  {
        file.write_all(lager[x].as_bytes()).expect("Failed to write");

        file.write_all(stout[x].as_bytes()).expect("Failed to write");

        file.write_all(non_alcoholic[x].as_bytes()).expect("Failed to write");

    }
    println!("\nSaved to file");
}