
use oracle::{Connection, ConnParam};

fn main() {
    println!("Conectando com  base Oracle");
    let conn = Connection::connect("SYS", "my-oracle-password", "", &[ConnParam::Sysdba]).expect("Não conseguiu riar conexão com base Oracle");
    match conn.ping() {
        Ok(_) => println!("Pingou com sucesso"),
        _ => println!("falhou em pingar o servidor..."),
    };

/*
    let sql = "SELECT ID FROM TB_TEST";
    let rows = conn.query(sql, &[]).unwrap();
    
    for row_result in &rows {
        let row = row_result.unwrap();
        let id: i32 = row.get("ID").unwrap();
        println!("ID = {}", id);
    }
*/
}
