# 🦀 Projeto: Compilador para Linguagem Rust

Este repositório contém o desenvolvimento de um analisador léxico e sintático para a linguagem **Rust**, desenvolvido para a disciplina de Compiladores utilizando a ferramenta **JavaCC**.

## 🚀 Fluxo de Trabalho (VS Code / Terminal)

Sempre que realizar alterações no arquivo de gramática (`.jj`), siga estes passos para atualizar o sistema:

## 1.Gerar:
```bash
java -cp javacc.jar javacc -OUTPUT_DIRECTORY=src/projeto_compilador_rust/ src/projeto_compilador_rust/Rust.jj
```

## 2.Compilar:
```bash
cd src
javac projeto_compilador_rust/*.java
```

## 3. Testar:
### Windows
```bash
Get-Content ../teste.rs | java projeto_compilador_rust.Rust
```
### Linux
```bash
java projeto_compilador_rust.Rust < ../teste.rs
```