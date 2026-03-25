# 🦀 Projeto: Compilador para Linguagem Rust

Este repositório contém o desenvolvimento de um analisador léxico e sintático para a linguagem **Rust**, desenvolvido para a disciplina de Compiladores utilizando a ferramenta **JavaCC**.

## 🚀 Fluxo de Trabalho (VS Code / Terminal)

Sempre que realizar alterações no arquivo de gramática (`.jj`), siga estes passos para atualizar o sistema:

### 1. Gerar os arquivos do JavaCC
Execute o comando na raiz do projeto para que o JavaCC processe a gramática:
```bash
java -cp javacc.jar javacc src/projeto_compilador_rust/Rust.jj