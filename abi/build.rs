fn main() {

    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto")
}