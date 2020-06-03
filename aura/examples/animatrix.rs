use rog_aura::{AniMeDbusWriter, AniMeMatrix, AniMePacketType};

fn main() {
    let mut writer = AniMeDbusWriter::new().unwrap();

    loop {
        for brightness in 0x01..0xFF {
            let mut matrix = AniMeMatrix::new();
            matrix.fill_with(brightness);

            let mut matrix: AniMePacketType = AniMePacketType::from(matrix);
            // println!("{:?}", matrix[0].to_vec());
            // println!("{:?}", matrix[1].to_vec());

            writer.write_image(&mut matrix).unwrap();
        }
        break;
    }

    // Try an outline, top and right
    let mut matrix = AniMeMatrix::new();
    {
        let mut tmp = matrix.get_mut();
        let mut first_row_done = false;
        for row in tmp.iter_mut() {
            if !first_row_done {
                for c in row.iter_mut() {
                    *c = 0xff;
                }
                first_row_done = true;
            } else {
                row[row.len() - 1] = 0xff;
            }
        }
    }

    let mut matrix: AniMePacketType = AniMePacketType::from(matrix);
    println!("{:?}", matrix[0].to_vec());
    println!("{:?}", matrix[1].to_vec());

    writer.write_image(&mut matrix).unwrap();
}
