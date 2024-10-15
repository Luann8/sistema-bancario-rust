use std::io;

struct Usuario {
    nome: String,
    data_nascimento: String,
    cpf: String,
    endereco: String,
    saldo: f64,
}

struct Conta<'a> {
    agencia: &'a str,
    numero_conta: usize,
    usuario: &'a Usuario,
}

impl<'a> Conta<'a> {
    fn depositar(&mut self, valor: f64, extrato: &mut String) {
        if valor > 0.0 {
            self.usuario.saldo += valor;
            extrato.push_str(&format!("Depósito:\tR$ {:.2}\n", valor));
            println!("\n=== Depósito realizado com sucesso! ===");
        } else {
            println!("\n@@@ Operação falhou! O valor informado é inválido. @@@");
        }
    }

    fn sacar(&mut self, valor: f64, extrato: &mut String, limite: f64, numero_saques: &mut usize, limite_saques: usize) {
        let excedeu_saldo = valor > self.usuario.saldo;
        let excedeu_limite = valor > limite;
        let excedeu_saques = *numero_saques >= limite_saques;

        if excedeu_saldo {
            println!("\n@@@ Operação falhou! Você não tem saldo suficiente. @@@");
        } else if excedeu_limite {
            println!("\n@@@ Operação falhou! O valor do saque excede o limite. @@@");
        } else if excedeu_saques {
            println!("\n@@@ Operação falhou! Número máximo de saques excedido. @@@");
        } else if valor > 0.0 {
            self.usuario.saldo -= valor;
            extrato.push_str(&format!("Saque:\t\tR$ {:.2}\n", valor));
            *numero_saques += 1;
            println!("\n=== Saque realizado com sucesso! ===");
        } else {
            println!("\n@@@ Operação falhou! O valor informado é inválido. @@@");
        }
    }

    fn exibir_extrato(&self, extrato: &str) {
        println!("\n================ EXTRATO ================");
        if extrato.is_empty() {
            println!("Não foram realizadas movimentações.");
        } else {
            println!("{}", extrato);
        }
        println!("Saldo:\t\tR$ {:.2}", self.usuario.saldo);
        println!("=========================================");
    }
}

fn menu() -> String {
    let menu = r#"
    ================ MENU ================
    [d]\tDepositar
    [s]\tSacar
    [e]\tExtrato
    [nc]\tNova conta
    [lc]\tListar contas
    [nu]\tNovo usuário
    [q]\tSair
    => "#;
    let mut input = String::new();
    print!("{}", menu);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn criar_conta<'a>(agencia: &'a str, numero_conta: usize, usuarios: &'a Vec<Usuario>) -> Option<Conta<'a>> {
    println!("Informe o CPF do usuário: ");
    let mut cpf = String::new();
    io::stdin().read_line(&mut cpf).expect("Failed to read line");

    if let Some(usuario) = usuarios.iter().find(|u| u.cpf.trim() == cpf.trim()) {
        println!("\n=== Conta criada com sucesso! ===");
        Some(Conta { agencia, numero_conta, usuario })
    } else {
        println!("\n@@@ Usuário não encontrado, fluxo de criação de conta encerrado! @@@");
        None
    }
}

fn listar_contas(contas: &Vec<Conta>) {
    for conta in contas {
        println!("====================================================================================================");
        println!("Agência:\t{}", conta.agencia);
        println!("C/C:\t\t{}", conta.numero_conta);
        println!("Titular:\t{}", conta.usuario.nome);
    }
}

fn main() {
    let limite_saques = 3;
    let agencia = "0001";
    let limite = 500.0;
    let mut extrato = String::new();
    let mut numero_saques = 0;
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut contas: Vec<Conta> = Vec::new();

    loop {
        let opcao = menu();

        match opcao.as_str() {
            "d" => {
                println!("Informe o valor do depósito: ");
                let mut valor = String::new();
                io::stdin().read_line(&mut valor).expect("Failed to read line");
                let valor: f64 = valor.trim().parse().unwrap_or_else(|_| {
                    println!("Por favor, insira um valor numérico válido.");
                    0.0
                });

                if let Some(conta) = contas.first_mut() {
                    conta.depositar(valor, &mut extrato);
                } else {
                    println!("\n@@@ Não há contas disponíveis para realizar depósito! @@@");
                }
            }
            "s" => {
                println!("Informe o valor do saque: ");
                let mut valor = String::new();
                io::stdin().read_line(&mut valor).expect("Failed to read line");
                let valor: f64 = valor.trim().parse().unwrap_or_else(|_| {
                    println!("Por favor, insira um valor numérico válido.");
                    0.0
                });

                if let Some(conta) = contas.first_mut() {
                    conta.sacar(valor, &mut extrato, limite, &mut numero_saques, limite_saques);
                } else {
                    println!("\n@@@ Não há contas disponíveis para realizar saque! @@@");
                }
            }
            "e" => {
                if let Some(conta) = contas.first() {
                    conta.exibir_extrato(&extrato);
                } else {
                    println!("\n@@@ Não há contas disponíveis para exibir extrato! @@@");
                }
            }
            "nu" => {
                println!("Informe o CPF (somente número): ");
                let mut cpf = String::new();
                io::stdin().read_line(&mut cpf).expect("Failed to read line");
                let cpf = cpf.trim().to_string();
                if !usuarios.iter().any(|u| u.cpf == cpf) {
                    println!("Informe o nome completo: ");
                    let mut nome = String::new();
                    io::stdin().read_line(&mut nome).expect("Failed to read line");
                    let nome = nome.trim().to_string();
                    println!("Informe a data de nascimento (dd-mm-aaaa): ");
                    let mut data_nascimento = String::new();
                    io::stdin().read_line(&mut data_nascimento).expect("Failed to read line");
                    let data_nascimento = data_nascimento.trim().to_string();
                    println!("Informe o endereço (logradouro, nro - bairro - cidade/sigla estado): ");
                    let mut endereco = String::new();
                    io::stdin().read_line(&mut endereco).expect("Failed to read line");
                    let endereco = endereco.trim().to_string();

                    usuarios.push(Usuario {
                        nome,
                        data_nascimento,
                        cpf,
                        endereco,
                        saldo: 0.0,
                    });

                    println!("=== Usuário criado com sucesso! ===");
                } else {
                    println!("@@@ Já existe usuário com esse CPF! @@@");
                }
            }
            "nc" => {
                let numero_conta = contas.len() + 1;
                if let Some(conta) = criar_conta(agencia, numero_conta, &usuarios) {
                    contas.push(conta);
                }
            }
            "lc" => listar_contas(&contas),
            "q" => break,
            _ => println!("Operação inválida, por favor selecione novamente a operação desejada."),
        }
    }
}
