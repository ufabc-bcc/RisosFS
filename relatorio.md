
# Universidade Federal do ABC

**2019.Q1 MCTA026-13 - Sistemas Operacionais**

**Professores:** [Emilio Francesquini](http://professor.ufabc.edu.br/~e.francesquini), [Fernando Teubl](http://professor.ufabc.edu.br/~fernando.teubl/)

**E-mail:** [e.francesquini@ufabc.edu.br](mailto:e.francesquini@ufabc.edu.br)

## Projeto de Programação - BrisaFS, um sistema de arquivos baseado em FUSE

**Alunos:** [Gustavo Murayama](mailto:gustavo.murayama@aluno.ufabc.edu.br), [Lucas Tornai de Carvalho](mailto:lucas.tornai@aluno.ufabc.edu.br)  
**RA:** 21028214, 21058912  
**Maio, 2019**

### Funcionalidades da Aplicação

O objetivo deste projeto foi o de desenvolver um sistema de arquivos (FS) utilizando a interface FUSE (Filesystem in Userspace), que pudesse executar as funções básicas de um FS, como criar arquivos, alterar permissões e persistir em disco. Inicialmente deveria ser utilizada a linguagem C e sua biblioteca FUSE, entretanto, neste projeto foi utilizada a linguagem Rust.

### Implementação

#### Linguagem Rust

A escolha da linguagem Rust foi feita por afinidade com a linguagem e por sua particularidade de segurança em questão de ponteiros, que não permite ao programador cometer erros básicos de difícil identificação como ocorre por exemplo na linguagem C. Por possuir uma biblioteca FUSE (apesar de incompleta se comparada à biblioteca em C), para as implementações necessárias neste projeto era o suficiente.

#### Conversão de C para Rust

A principal diferença entre a implementação em C para a implementação em Rust foi o vetor de blocos de memória. No BrisaFS disponibilizado pelo professor, é utilizado uma mecânica com dois ponteiros que apontavam para este mesmo vetor, porém o primeiro era um ponteiro de bytes enquanto outro era um ponteiro de struct de inodes. Em Rust, isto não era possível, e portanto foram criadas duas structs, uma de inodes e uma de blocos de memória, e dois vetores separados para cada uma das structs.
Isto ocorre pois Rust possui o sistema de [Ownership](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)[2], que é um mecanismo que entre outras coisas previne o uso irresponsável de ponteiros, o que restringe este tipo de manobra que pode ser feita em linguagens como C.
O restante da conversão apenas se deu pela tradução direta das funções entre as linguagens, uma vez que algumas funções do FUSE ainda não estavam implementadas, usando como base ao início um guia para utilização da biblioteca FUSE em Rust disponível no [24 Days of Rust](https://zsiciarz.github.io/24daysofrust/book/vol1/day15.html)[1].

#### Sistema de arquivos

Para a implementação do sistema de arquivos, como dito na seção anterior, foram criadas duas structs: `Inode` e `MemoryBlock`. A struct Inode guarda os dados de Inodes, enquanto a struct MemoryBlock guarda os dados de arquivos em bytes (os dados propriamente ditos).

Para implementação das funções do FS, foram implementadas as funções pré-determinadas da interface FUSE.

Já para a persistência, que deveria ser feita em um arquivo, são utilizados dois arquivos. Um deles, `.inodes.risos`, guarda o vetor de inodes, enquanto o arquivo `.disco.risos` guarda o vetor de dados do FS. Ponto importante é a serialização destes dados para salvar num arquivo em disco, que será discutido em uma próxima seção.

O arquivo `main.rs` possui as funções necessárias para o funcionamento do sistema, ou seja, a implementação da interface FUSE e a função `main()`, que executa o sistema. O arquivo `persistence.rs` é uma abstração de um "disco virtual", possuindo as funções para persistência do FS em um arquivo no disco e funções auxiliares a isto, como o vetor de `Inode` e o vetor de `MemoryBlock`, a leitura/escrita nos arquivos que representam este disco e funções para encontrar blocos livres.

#### Biblioteca serde

A biblioteca utilizada para a serialização e desserialização das structs foi a [serde](https://serde.rs/)[3], que é bem estabelecida e consegue serializar e deserializar qualquer implementação que contenha tipos primitivos.

Para isto, no entanto foi necessária a serialização de todas as structs contidas na struct FileAttr, inerente à biblioteca rust, e isto foi feito no arquivo `serializarion.rs`, que é o arquivo no qual são serializadas as structs internas para permitir a serialização da externa.

### Compilação e execução

Antes de mais nada é necessária a instalação do [Rustup](https://www.rust-lang.org/learn/get-started), instalador e gerenciador de versão do Rust.

Para macOS ou Linux basta executar num terminal:
```
curl https://sh.rustup.rs -sSf | sh
```

Para compilar o programa, deve-se entrar no diretório raíz e executar o comando: `cargo build`. `Cargo` é o gerenciador de pacotes do rust. `build` é o comando para compilar o programa do diretório atual, isto faz com que seja gerada uma pasta `target` com os arquivos que serão utilizados para execução propriamente dita do programa.

Para a execução, deve-se utilizar o comando `cargo run <diretório>` dentro da pasta raíz, onde <diretório> é onde se deseja executar o FS. `run` é o comando que executa o programa se utilizando dos arquivos gerados na pasta target.

Para executar a última versão lançada, acessar a página de [releases](https://github.com/ufabc-bcc/2019_Q1_SO_BrisaFS-risosfs/releases) e fazer o download do arquivo `risos_fs`.

####  Funcionamento do programa

A primeira vez que o programa for rodado, ele iniciará todos os processos para criação do `.inode.risos` e `.disco.risos`, alocará a memória necessária e rodará automaticamente algumas funções do FS.

```
Done =)

Tamanho do disco (kbytes): 1048576
Tamanho do bloco de memória (kbytes): 2432
Quantidade máxima de arquivos (Inode 2432 bytes): 1024
RisosFS started!
lookup(parent=1, name="BDMV")
getattr(ino=1)
lookup(parent=1, name=".xdg-volume-info")
lookup(parent=1, name="autorun.inf")
lookup(parent=1, name=".Trash")
readdir(ino=1, fh=0, offset=0)
lookup(parent=1, name=".Trash-1000")
readdir(ino=1, fh=0, offset=1)
getattr(ino=1)
readdir(ino=1, fh=0, offset=0)
readdir(ino=1, fh=0, offset=1)
lookup(parent=1, name="autorun.inf")
getattr(ino=1)
```

Para uma melhor visualização do que está sendo chamado, o código executa um `println!` com algumas informações pertinentes. Abaixo estão alguns exemplos de execução de comandos no terminal e o resultado de cada operação.

**Terminal**
```
$ ls -la
total 4
drwxr-xr-x 0 root      root         0 May  5 21:05 .
drwxr-xr-x 3 gmurayama gmurayama 4096 May  2 23:37 
```
**RisosFS**
```
readdir(ino=1, fh=0, offset=0)
getattr(ino=1)
readdir(ino=1, fh=0, offset=1)
```

**Terminal**
```
$ echo teste > risos_fs
```

**RisosFS**
```
readdir(ino=1, fh=0, offset=1)
lookup(parent=1, name="risos_fs")
create(name="risos_fs", mode=33188, flags=33345)
write(ino=2, offset=0, data=6)
```

**Terminal**
```
$ mkdir dir_rs
```

**RisosFS**
```
getattr(ino=1)
lookup(parent=1, name="dir_rs")
    - lookup(name="dir_rs", name_from_inode="risos_fs", equals=false)
```

Cada struct de `Inode` possui um vetor de `references`. No caso de uma pasta, esse vetor guarda o número `ino` de cada inode pertencente a esse diretório (que vamos nos referir como "inode pai").

Quando é acionado o método de `readdir`, no parâmetro `parent` é passado o número `ino` do inode pai e é feito uma varredura no disco para saber quais os arquivos que devemos mostrar.

O `create` cria um novo arquivo e salva o Inode e o seu conteúdo (inicialmente vazio) no disco virtual. No método `write`, através do `ino` passado como argumento, localizamos o arquivo criado e escrevemos o conteúdo de array de bytes `[u8]` no `MemoryBlock` correspondente ao Inode.

Para interromper a execução do RisosFS, é necessário dar unmount no diretório utilizado. No Linux, o comando é:

```
fusermount -u <directory>
```

### Limitações

- Há um limite do quanto o arquivo pode ter de tamanho. Atualmente, o arquivo pode ter no máximo 2432 kbytes (~ 2MB).
- O número máximo de arquivos que podem existir no disco virtual é 1024.
- Não é possível criar links simbólicos
- Se o Filesystem for interrompido de maneira inesperada, os dados não são salvos. Apenas é salvo quando dado o unmount apropriado.
- Não há escalabilidade no FS como está atualmente, sem o aumento arbitrário do disco e os arquivos puderam ocupar mais de um bloco de memória. Seria necessário resolver as issues [#7](https://github.com/ufabc-bcc/2019_Q1_SO_BrisaFS-risosfs/issues/7), [#8](https://github.com/ufabc-bcc/2019_Q1_SO_BrisaFS-risosfs/issues/8), [#9](https://github.com/ufabc-bcc/2019_Q1_SO_BrisaFS-risosfs/issues/9) e [#10](https://github.com/ufabc-bcc/2019_Q1_SO_BrisaFS-risosfs/issues/10).

### Biliografia
[1]: [24 Days of Rust](https://zsiciarz.github.io/24daysofrust/)  
[2]: [The Rust Programming Language Book](https://doc.rust-lang.org/stable/book/title-page.html)  
[3]: [serde](https://serde.rs/)  
