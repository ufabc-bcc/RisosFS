
# Universidade Federal do ABC

**2019.Q1 MCZA020-13 - Programação Paralela**

**Professores:** [Emilio Francesquini](http://professor.ufabc.edu.br/~e.francesquini), [Fernando Teubl](http://professor.ufabc.edu.br/~fernando.teubl/)

**E-mail:** [e.francesquini@ufabc.edu.br](mailto:e.francesquini@ufabc.edu.br)

## Projeto de Programação - BrisaFS, um sistema de arquivos baseado em FUSE

**Alunos:** [Gustavo Murayama](mailto:gustavo.murayama@aluno.ufabc.edu.br), [Lucas Tornai de Carvalho](mailto:lucas.tornai@aluno.ufabc.edu.br)
**RA:** 21028214, 21058912

**Abril, 2019**

### Funcionalidades da Aplicação

O objetivo deste projeto foi o de desenvolver um sistema de arquivos (FS) utilizando a interface FUSE (Filesystem in Userspace), que pudesse executar as funções básicas de um FS, como criar arquivos, alterar permissões e persistir em disco. Inicialmente deveria ser utilizada a linguagem C e sua biblioteca FUSE, entretanto, neste projeto foi utilizada a linguagem Rust.

### Implementação

#### Linguagem Rust

A escolha da linguagem Rust foi feita por afinidade com a linguagem e por sua particularidade de segurança em questão de ponteiros, onde não permite ao programador cometer erros básicos de difícil identificação como ocorre por exemplo na linguagem C. Por possuir uma biblioteca FUSE (apesar de incompleta se comparada à biblioteca em C), para as implementações necessárias neste projeto era o suficiente.

### Conversão de C para Rust

O principal ponto de diferença para traduzir de C para Rust foi no vetor de blocos de memória, onde, na implementação em C disponibilizada pelo professor, se utilizada de uma mecânica de ponteiros onde foram criados dois ponteiros que apontavam para este mesmo vetor, porém um era um ponteiro de bytes enquanto outro era um ponteiro de inodes. Em rust, isto não era possível, e portanto foram criadas duas structs, uma de inodes e uma de blocos de memória, e dois vetores separados para cada uma das structs.
O restante apenas se deu pela tradução direta das funções entre as linguagens, já que as funções principais do FUSE ainda não estavam implementadas.

### Sistema de arquivos

Para a implementação do sistema de arquivos, como dito na seção anterior, foram criadas duas structs, `Inode` e `MemoryBlock`, sendo que a struct Inode guarda os dados de Inodes, enquanto a struct MemoryBlock guarda os dados de arquivos (os dados propriamente ditos).

Para implementação das funções do FS foram implementadas as funções pré-determinadas da interface FUSE.

Já para a persistência, que deveria ser feita em um arquivo, na verdade são utilizados dois arquivos. Um deles, `.inodes.risos` guarda o vetor de inodes, enquanto o arquivo `.disco.risos` guarda o vetor de dados do FS. Ponto importante é a serialização destes dados para salvar num arquivo em disco, que será discutido em uma próxima seção.

O arquivo `main.rs` possui as funções necessárias para o funcionamento do sistema, ou seja, a implementação da interface FUSE e a função `main()`, que executa o sistema. O arquivo `persistence.rs` possui as funções para persistência do FS em um arquivo no disco e funções auxiliares a isto, como a implementação do "disco", a leitura/escrita nos arquivos que representam este disco e funções para encontrar blocos livres.

## Biblioteca serde

A biblioteca utilizada para a serialização e desserialização das structs foi a [serde](https://serde.rs/), que é bem estabelecida e consegue serializar e deserializar qualquer implementação que contenha tipos primitivos.

Para isto, no entanto foi necessária a serialização de todas as structs contidas na struct FileAttr, inerente à biblioteca rust, e isto foi feito no arquivo `serializarion.rs`, que é o arquivo no qual são serializadas as structs internas para permitir a serialização da externa.

### Compilação e execução

Antes de mais nada é necessária a instalação do [Rustup](https://www.rust-lang.org/learn/get-started), instalador e gerenciador de versão do Rust.

Para macOS ou Linux basta executar num terminal:
```
curl https://sh.rustup.rs -sSf | sh
```

Para compilar o programa, deve-se entrar no diretório raíz e executar o comando: `cargo build`. `Cargo` é o gerenciador de pacotes do rust. `build` é o comando para compilar o programa do diretório atual, isto faz com que seja gerada uma pasta `target` com os arquivos que serão utilizados para execução propriamente dita do programa.

Para a execução, deve-se utilizar o comando `cargo run <diretório>` dentro da pasta raíz, onde <diretório> é onde se deseja executar o FS. `run` é o comando que executa o programa se utilizando dos arquivos gerados na pasta target.

###  Funcionamento do programa

### Limitações