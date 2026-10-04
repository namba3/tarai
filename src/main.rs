fn main() {
    let input = (10, 5, 0);
    println!(
        "tarai_naive: {}",
        tarai::tarai_naive(input.0, input.1, input.2)
    );
    println!(
        "tarai_memo: {}",
        tarai::tarai_memo(input.0, input.1, input.2)
    );
    println!(
        "tarai_lazy_closure: {}",
        tarai::tarai_lazy_closure(input.0, input.1, input.2)
    );
    println!(
        "tarai_lazy_enum: {}",
        tarai::tarai_lazy_enum(input.0, input.1, input.2)
    );
}
