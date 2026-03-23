PARSER_BEGIN(Rust)
package projeto_compilador_rust;

public class Rust {
  public static void main(String args []) throws ParseException {
    Rust parser = new Rust(System.in);
    System.out.println("Compilador Rust carregado. Digite o codigo:");
    try {
      parser.Start(); // Membro 1: Ponto de entrada
      System.out.println("Analise Sintatica: OK.");
    } catch (ParseException e) {
      System.out.println("Erro Sintatico: " + e.getMessage());
    }
  }
}
PARSER_END(Rust)

/* ------------------------------------------------------------------------- */
/* LEXICO (Membro 1) - Adicionar os tokens de todos conforme avancam         */
/* ------------------------------------------------------------------------- */
SKIP : { " " | "\r" | "\t" | "\n" }

TOKEN : {
    < FN: "fn" > | < LET: "let" > | < MUT: "mut" > | < MATCH: "match" >
  | < IF: "if" > | < ELSE: "else" > | < WHILE: "while" > | < FOR: "for" >
  | < ARROW: "->" > | < ASSIGN: "=" > | < SEMICOLON: ";" > | < COLON: ":" >
  | < LBRACE: "{" > | < RBRACE: "}" > | < LPAREN: "(" > | < RPAREN: ")" >
  | < COMMA: "," > | < FAT_ARROW: "=>" >
}

TOKEN : {
    < I32: "i32" > | < F64: "f64" > | < BOOL: "bool" >
  | < IDENTIFIER: (["a"-"z", "A"-"Z", "_"])+ (["a"-"z", "A"-"Z", "0"-"9", "_"])* >
  | < INTEGER_LITERAL: (["0"-"9"])+ >
}

/* ------------------------------------------------------------------------- */
/* SINTATICO - Divisao por Membros                                           */
/* ------------------------------------------------------------------------- */

/** MEMBRO 1: Estrutura Global **/
void Start() : {}
{
  ( Funcao() )* <EOF>
}

/** MEMBRO 3: Funcoes e Structs **/
void Funcao() : {}
{
  <FN> <IDENTIFIER> <LPAREN> [ Parametros() ] <RPAREN> [ <ARROW> Tipo() ] Bloco()
}

void Bloco() : {}
{
  <LBRACE> ( Comando() )* <RBRACE>
}

/** MEMBRO 1 (Integrador): Orquestra os comandos dos outros membros **/
void Comando() : {}
{
    LOOKAHEAD(2) DeclaracaoVariavel() <SEMICOLON>
  | EstruturaControle()
  | Expressao() <SEMICOLON>
}

/** MEMBRO 2: Variaveis e Tipagem **/
void DeclaracaoVariavel() : {}
{
  <LET> [ <MUT> ] <IDENTIFIER> [ <COLON> Tipo() ] <ASSIGN> Expressao()
}

void Tipo() : {}
{
  <I32> | <F64> | <BOOL> | <IDENTIFIER>
}

/** MEMBRO 4: Fluxo de Controle (if, while, match) **/
void EstruturaControle() : {}
{
    <IF> Expressao() Bloco() [ <ELSE> Bloco() ]
  | <WHILE> Expressao() Bloco()
  | <MATCH> Expressao() <LBRACE> ( <INTEGER_LITERAL> <FAT_ARROW> Bloco() )* <RBRACE>
}

/** MEMBRO 5: Expressoes e Operadores **/
// Aqui voce pode reaproveitar sua logica de soma/termo do exemplo anterior
void Expressao() : {}
{
  Termo() ( ( <PLUS> | <MINUS> ) Termo() )*
}

void Termo() : {}
{
  Fator() ( ( <MULTIPLY> | <DIVIDE> ) Fator() )*
}

void Fator() : {}
{
  <INTEGER_LITERAL> | <IDENTIFIER> | <LPAREN> Expressao() <RPAREN>
}