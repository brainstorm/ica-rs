use progenitor::generate_api;

fn main() {
    generate_api!(spec = "openapi/gds.json");
}
