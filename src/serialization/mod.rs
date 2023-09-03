use std::{fs::File, io::{Write, Read}, error::Error};

pub fn serializeu64(file_path: &str, item: &Vec<u64>) -> Result<(), Box<dyn Error>>{
    let mut stream: File = File::create(file_path)?;
    //let mut buffer: [u8; 16] = [0; 16];
    
    //let limit: usize = item.len() / 2;

    let mut test: Vec<u8> = Vec::new();

    for i in item {
        test.push((*i >> 56) as u8);
        test.push((*i >> 48) as u8);
        test.push((*i >> 40) as u8);
        test.push((*i >> 32) as u8);
        test.push((*i >> 24) as u8);
        test.push((*i >> 16) as u8);
        test.push((*i >>  8) as u8);
        test.push((*i >>  0) as u8);
    }
    stream.write(&test)?;


    /*
    for i in 0..limit  {
        let index: usize = i * 2;
        buffer[0] = (item[index] >> 56 ) as u8;
        buffer[1] = (item[index] >> 48 ) as u8;
        buffer[2] = (item[index] >> 40) as u8;
        buffer[3] = (item[index] >> 32) as u8;
        buffer[4] = (item[index] >> 24) as u8;
        buffer[5] = (item[index] >> 16) as u8;
        buffer[6] = (item[index] >>  8) as u8;
        buffer[7] = (item[index] >>  0) as u8;
        
        let index: usize = i * 2 + 1;
        buffer[ 8] = (item[index] >> 56 ) as u8;
        buffer[ 9] = (item[index] >> 48 ) as u8;
        buffer[10] = (item[index] >> 40) as u8;
        buffer[11] = (item[index] >> 32) as u8;
        buffer[12] = (item[index] >> 24) as u8;
        buffer[13] = (item[index] >> 16) as u8;
        buffer[14] = (item[index] >>  8) as u8;
        buffer[15] = (item[index] >>  0) as u8;
        stream.write(&buffer)?;
    }

    for i in item {
        buffer[0] = (*i >> 56) as u8;
        buffer[1] = (*i >> 48) as u8;
        buffer[2] = (*i >> 40) as u8;
        buffer[3] = (*i >> 32) as u8;
        buffer[4] = (*i >> 24) as u8;
        buffer[5] = (*i >> 16) as u8;
        buffer[6] = (*i >>  8) as u8;
        buffer[7] = (*i >>  0) as u8;
        
        stream.write(&buffer)?;
    }
    */

    Ok(())
}


pub fn deserializeu64(file_path: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut output: Vec<u64> = Vec::new();

    let mut stream: File = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let limit: usize = buffer.len()/8;
    for i in 0..limit {
        let index: usize = i * 8;
        output.push(
            (buffer[index + 0] as u64) << 56 |
            (buffer[index + 1] as u64) << 48 |
            (buffer[index + 2] as u64) << 40 |
            (buffer[index + 3] as u64) << 32 |
            (buffer[index + 4] as u64) << 24 |
            (buffer[index + 5] as u64) << 16 |
            (buffer[index + 6] as u64) <<  8 |
            (buffer[index + 7] as u64)
        );
    }
    
    /* 
    let length: u64 = stream.metadata()?.len()/8;
    
    let mut buffer: [u8; 8] = [0; 8];
    
    for i in 0..length {
        stream.read_at(&mut buffer, i * 8)?;
        
        output.push(
            (buffer[0] as u64) << 56 |
            (buffer[1] as u64) << 48 |
            (buffer[2] as u64) << 40 |
            (buffer[3] as u64) << 32 |
            (buffer[4] as u64) << 24 |
            (buffer[5] as u64) << 16 |
            (buffer[6] as u64) <<  8 |
            (buffer[7] as u64)
        );
    }
    */

    Ok(output)
}