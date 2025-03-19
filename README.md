# Módulo em Rust para análise de séries temporais com regressão linear.
Este projeto implementa um módulo em Rust para análise de séries temporais usando regressão linear. Ele permite ajustar um modelo de regressão linear a dados, realizar previsões e calcular métricas de avaliação, tudo de forma "pura" (sem dependências externas para cálculos matemáticos).

## Estrutura do Projeto

- **`src/main.rs`**: Contém o módulo `time_series`, a função `main` com um exemplo de uso e testes unitários.
- **`Cargo.toml`**: Define o projeto Rust (versão 0.1.0, edição 2021).
- **`.gitignore`**: Ignora o diretório `target/` gerado pelo Cargo.
- **`README.md`**: Este arquivo, com instruções e descrição.

## Como Usar
1. Clone o repositório:
   ```bash
   git clone <https://github.com/rxiozin/time-wise-analytics>
   cd time-wise-analytics

2. Execute o exemplo
   ```bash
   cargo run 

3. Execute os testes:
   ```bash
   cargo test


# Funcionalidades Implementadas
O módulo contém a estrutura `LinearRegression` e as seguintes funções:<br>
<br>
`Linear Regression`
Estrutura que armazena o modelo
   ```rust
  pub struct LinearRegression {
    pub slope: f64,      // Inclinação da reta
    pub intercept: f64,  // Intercepto no eixo y
   }
```
## Funções
- **`fit`**: Pega seus dados e calcula a melhor reta que passa por eles
- **`predict`**: Usa a reta pra prever um valor novo.
- **`r_squared`**: Diz o quão boa a reta é para explicar seus dados (0 a 1).
- **`mean_squared_error`**: Mostra o erro médio entre os dados reais e a reta.

## Exemplo de Resultado
```bash
   Inclinação: 1.9900
   Intercepto: 0.0500
   Previsão para x=6: 11.9900
   R²: 0.9984
   MSE: 0.0080
```
## Testes

Os testes estão no final do `src/main.rs` e checam:

- **Se a reta é calculada direito**
- **Se as previsões batem**
- **Se as métricas estão certas**
- **Se ele reclama quando os dados estão errados**
## Para mudar
- **Troque os dados no exemplo para usar os seus.**
- **Teste previsões com outros valores**

*_Feito por Vinicius Raio dos Santos em Março de 2025._*


