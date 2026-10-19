[English](README.md) | **Português** | [Español](README.es.md)

# Horae - Monitor de estudos no terminal

Um monitor de estudos para o terminal, feito com Rust e Ratatui.

![horae timer screen](assets/tracking-your-time.png)

## Funcionalidades

- **Cronômetro com dígitos grandes** — o tempo da sessão é exibido em caracteres de bloco, com o total do dia logo abaixo
- **Matérias** — organize as sessões por matéria; a matéria padrão "General" está sempre disponível
- **Sessões de estudo** — cada sessão é salva no SQLite com horário de início, horário de término e duração
- **Tarefas** — lista de afazeres simples com título, descrição, prioridade e status (concluída/aberta)
- **Análises** — gráfico de barras semanal das horas de estudo e o histórico completo de sessões anteriores

## Atalhos de teclado

### Cronômetro (ocioso)

| Tecla | Ação |
|-----|--------|
| `Space` | Abrir o seletor de matéria para iniciar uma sessão |
| `a` | Abrir análises |
| `t` | Abrir tarefas |
| `s` | Abrir matérias |
| `q` | Sair |

### Cronômetro (estudando)

| Tecla | Ação |
|-----|--------|
| `Space` | Parar a sessão atual |
| `t` | Abrir tarefas |
| `q` | Sair |

### Seletor de matéria

| Tecla | Ação |
|-----|--------|
| `j` / `↓` | Mover para baixo |
| `k` / `↑` | Mover para cima |
| `Enter` | Iniciar a sessão com a matéria selecionada |
| `Esc` / `q` | Cancelar |

### Análises / Tarefas / Matérias

| Tecla | Ação |
|-----|--------|
| `j` / `↓` | Mover para baixo |
| `k` / `↑` | Mover para cima |
| `l` / `→` | Expandir o item selecionado |
| `h` / `←` / `Esc` | Recolher / voltar |
| `n` | Novo item (tarefas e matérias) |
| `d` | Excluir o item selecionado |
| `Space` | Alternar o status da tarefa (somente em tarefas) |
| `q` | Sair |

## Compilar e executar

Requer Rust 1.88 ou superior.

```sh
cargo run --release
```

O banco de dados SQLite (`horae.db`) é criado no diretório de trabalho atual na primeira execução.
