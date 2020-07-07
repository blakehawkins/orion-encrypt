use std::io::Read;

#[paw::main]
fn main(args: paw::Args) -> std::io::Result<()> {
    let args = args.collect::<Vec<_>>();
    if args.len() != 2 || args.get(0).unwrap() == "--help" {
        println!("Usage: orion-encrypt \"pass phrase\" < plaintext > ciphertext");
        std::process::exit(1);
    }

    let pass_phrase = format!("{:<32}", args.into_iter().nth(1).unwrap());

    let key = orion::aead::SecretKey::from_slice(&pass_phrase.as_bytes()[0..32])
        .expect("failed to build secret key from pass phrase");

    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf)?;

    let ciphertext = orion::aead::seal(&key, buf.as_bytes()).expect("failed to seal");

    println!("{}", base64::encode(ciphertext));

    Ok(())
}
