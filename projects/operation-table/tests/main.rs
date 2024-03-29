use operation_table::{OperationKind, OperationTable, TableDisplay};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn mul_odd_even() {
    println!("$$");
    let m = TableDisplay::default();
    println!("{}", m);
    println!("$$");
}
#[test]
fn test_base7() {
    println!("$$");
    let m = OperationTable::default().with_base(7).with_operation(OperationKind::Addition);
    println!("{}", m.display());
    println!("$$");
    println!("$$");
    let m = OperationTable::default().with_base(7);
    println!("{}", m.display());
    println!("$$");
    println!("$$");
    let m = OperationTable::default().with_base(7).with_display(7).with_operation(OperationKind::Addition);
    println!("{}", m.display());
    println!("$$");
    println!("$$");
    let m = OperationTable::default().with_base(7).with_display(7);
    println!("{}", m.display());
    println!("$$");
}

#[test]
fn test_base12() {
    println!("$$");
    let m = OperationTable::default().with_base(12).with_operation(OperationKind::Enumerate).with_min(0).with_display(12);
    println!("{}", m.display());
    println!("$$");
    println!("$$");
    let m = OperationTable::default().with_base(12).with_operation(OperationKind::Addition).with_min(1).with_display(12);
    println!("{}", m.display());
    println!("$$");
    println!("$$");
    let m = OperationTable::default().with_base(12).with_display(12).with_upper_triangle(true);
    println!("{}", m.display());
    println!("$$");
}
