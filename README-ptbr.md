# Horae - Rastreador de estudos no terminal

[English](README.md) | [Русский](README-ru.md) | [中文](README-zh.md) | **Português (BR)** | [Español](README-es.md)

Um rastreador de estudos no terminal feito com Rust e Ratatui.

![tela do cronômetro do horae](assets/tracking-your-time.png)

## Funcionalidades

- **Cronômetro com dígitos grandes** — o tempo da sessão é exibido em caracteres de bloco, com o total diário logo abaixo
- **Matérias** — organize as sessões por matéria; uma matéria padrão "General" está sempre disponível
- **Sessões de estudo** — cada sessão é salva no SQLite com horário de início, horário de término e duração
- **Tarefas** — lista de afazeres simples com título, descrição, prioridade e status concluída/aberta
- **Análises** — gráfico de barras semanal das horas de estudo e o histórico completo das sessões anteriores

## Atalhos de teclado

### Cronômetro (parado)

| Tecla | Ação |
|-----|--------|
| `Space` | Abre o seletor de matérias para iniciar uma sessão |
| `a` | Abre as análises |
| `t` | Abre as tarefas |
| `s` | Abre as matérias |
| `q` | Sai |

### Cronômetro (estudando)

| Tecla | Ação |
|-----|--------|
| `Space` | Encerra a sessão atual |
| `t` | Abre as tarefas |
| `q` | Sai |

### Seletor de matérias

| Tecla | Ação |
|-----|--------|
| `j` / `↓` | Move para baixo |
| `k` / `↑` | Move para cima |
| `Enter` | Inicia a sessão com a matéria selecionada |
| `Esc` / `q` | Cancela |

### Análises / Tarefas / Matérias

| Tecla | Ação |
|-----|--------|
| `j` / `↓` | Move para baixo |
| `k` / `↑` | Move para cima |
| `l` / `→` | Expande o item selecionado |
| `h` / `←` / `Esc` | Recolhe / volta |
| `n` | Novo item (tarefas e matérias) |
| `d` | Exclui o item selecionado |
| `Space` | Alterna o status da tarefa (apenas tarefas) |
| `q` | Sai |

## Compilar e executar

Requer Rust 1.88 ou superior.

```sh
cargo run --release
```

O banco de dados SQLite (`horae.db`) é criado no diretório de trabalho atual na primeira execução.
