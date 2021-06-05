# MBS TUI
## TUI simples para o consumo da [API](https://github.com/leitura/api_rest_ads)

[![Chat on Telegram](https://img.shields.io/badge/Telegram-OMentalista-blue)](https://t.me/OMentalista)
### Pré-requisitos

Você irá precisar destes softwares instalados em sua máquina para rodar localmente:
- [Git](https://git-scm.com)
- [Rust Language](https://www.rust-lang.org)


## "Instalação"

#### Primeiro, instale o ```build-essential``` em sua máquina

#### Hospede a  [API](https://github.com/leitura/api_rest_ads), após vá no arquivo: src/reqs/mod.rs , linha 25 e substitua pela a URL de sua [API](https://github.com/leitura/api_rest_ads)

#### Clone o repositório
```bash
$ git clone https://github.com/leitura/client-api.git && cd client-api
```

### Compilando
```bash
$ cargo build
# Gerará um binário nomeado "client" que estará em target/debug
# caso queira rodar diretamente, use cargo run
```