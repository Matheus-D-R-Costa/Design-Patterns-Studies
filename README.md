# Design Patterns Studies

Este repositório contém implementações em Rust dos principais padrões de design de software. O objetivo é fornecer exemplos práticos e claros de cada padrão, seguindo boas práticas de programação e as características idiomáticas do Rust.

## Organização do Repositório

O projeto está organizado em três categorias principais de padrões de design:

### Padrões Criacionais (Creational Patterns)

Padrões que lidam com mecanismos de criação de objetos:

- [Em desenvolvimento]

### Padrões Estruturais (Structural Patterns)

Padrões que lidam com composição de classes e objetos:

- [Adapter (Adaptador)](structural_patterns/adapter): Permite que interfaces incompatíveis trabalhem juntas.

### Padrões Comportamentais (Behavioral Patterns)

Padrões que lidam com comunicação entre objetos:

- [Strategy (Estratégia)](behavioral_patterns/strategy): Define uma família de algoritmos encapsulados e intercambiáveis.
- [Observer (Observador)](behavioral_patterns/observer): Define uma dependência um-para-muitos entre objetos.

## Como Executar os Exemplos

Cada padrão de design é implementado como um projeto Rust separado. Para executar um exemplo específico:

1. Navegue até o diretório do padrão desejado
2. Execute o projeto usando Cargo:

```bash
cd behavioral_patterns/strategy
cargo run
```

## Princípios de Design Seguidos

Este projeto segue diversos princípios de design:

- **SOLID**:
  - Princípio da Responsabilidade Única (SRP)
  - Princípio Aberto-Fechado (OCP)
  - Princípio da Substituição de Liskov (LSP)
  - Princípio da Segregação de Interface (ISP)
  - Princípio da Inversão de Dependência (DIP)

- **Outros Princípios**:
  - Composição sobre herança
  - Programar para interfaces, não implementações
  - Encapsulamento de comportamentos
  - Lei de Demeter (princípio do menor conhecimento)

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para:
- Implementar novos padrões de design
- Melhorar implementações existentes
- Adicionar testes
- Melhorar a documentação

## Licença

Este projeto está licenciado sob [CC0 1.0 Universal](LICENSE) - veja o arquivo LICENSE para detalhes. 