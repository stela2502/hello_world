//use needletail::parse_fastx_file;
use kmers::naive_impl::Kmer;

fn main() {
    let l = 1;
    
    println!("Internal representations of numbers: \nu8  {l:#008b}\nu16 {l:#016b}\nu32 {l:#032b}\nu64 {l:#064b}\n");

    let seq = b"AGCT";

    println!("The sequence we look at: {seq:?} and the binary version:");
    for nuc in seq {
        println!( "{:#08b}", nuc );
    }

    let sv =  Kmer::from(seq);
    println!( "2bit encoded AGCT looks like that: {:#64b}", sv.into_u64() );

    let sv2 =  Kmer::from(b"AAAACCCCGGGGTTTTAAAACCCCGGGGTTTT");
    println!( "AAAACCCCGGGGTTTTAAAACCCCGGGGTTTT looks like that: {:#64b}", sv2.into_u64() );

    println!("8 bit slices:");
    for bit in sv2.into_u64().to_le_bytes() {
        println!( "{:#64b}", bit );
    }

    println!("Now I read our test fastq database to create the indices");

    let mut genes:GeneIds = GeneIds::new(32); 

    /*
    >Cd3e|ENSMUST00000102832.1|chr9:44998738 site_id: user_seq_28_project_mouseImmune, AMPLICON
    ACAGGTCCTGCCCCATTTATAGATCCTGGCCCAGCCCCTGCCACAGGTGCCTCTCCAGAT
    TTCCCCTTAGATCCTCGGATGGTCATCTCCATCTCCATGAATACACCAGCCCCCTCTCTG
    CTAATGCAAAAGGCAATAAAGTGTATTGGCTGG
    */
    genes.add( b"ACAGGTCCTGCCCCATTTATAGATCCTGGCCCAGCCCCTGCCACAGGTGCCTCTCCAGATTTCCCCTTAGATCCTCGGATGGTCATCTCCATCTCCATGAATACACCAGCCCCCTCTCTGCTAATGCAAAAGGCAATAAAGTGTATTGGCTGG", "Cd3e".to_string() );




}