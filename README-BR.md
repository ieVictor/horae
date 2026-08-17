# Horae - Rastreador de estudo para terminal

Um rastreador de estudo para terminal construído com Rust e Ratatui.

![horae timer screen](assets/tracking-your-time.png)

## Funcionalidades

- **Temporizador com dígitos grandes** — tempo da sessão exibido em caracteres de bloco, total diário mostrado abaixo
- **Assuntos** — organize sessões por assunto; um padrão "General" sempre está disponível
- **Sessões de estudo** — cada sessão é salva no SQLite com hora de início, hora de término e duração
- **Tarefas** — lista de afazeres leve com título, descrição, prioridade e status concluído/aberto
- **Análise** — gráfico de barras semanais de horas de estudo mais um histórico completo de sessões passadas

## Atalhos de teclado

### Temporizador (ocioso)

| Tecla | Ação |
|-------|------|
| `Space` | Abrir seletor de assunto para iniciar uma sessão |
| `a` | Abrir análise |
| `t` | Abrir tarefas |
| `s` | Abrir assuntos |
| `q` | Sair |

### Temporizador (estudando)

| Tecla | Ação |
|-------|------|
| `Space` | Parar sessão atual |
| `t` | Abrir tarefas |
| `q` | Sair |

### Seletor de assunto

| Tecla | Ação |
|-------|------|
| `j` / `↓` | Mover para baixo |
| `k` / `↑` | Mover para cima |
| `Enter` | Iniciar sessão com assunto selecionado |
| `Esc` / `q` | Cancelar |

### Análise / Tarefas / Assuntos

| Tecla | Ação |
|-------|------|
| `j` / `↓` | Mover para baixo |
| `k` / `↑` | Mover para cima |
| `l` / `→` | Expandir item selecionado |
| `h` / `←` / `Esc` | Recolher / voltar |
| `n` | Novo item (tarefas e assuntos) |
| `d` | Deletar item selecionado |
| `Space` | Alternar status da tarefa (apenas tarefas) |
| `q` | Sair |

## Compilar e executar

Requer Rust 1.88 ou posterior.

```sh
cargo run --release
```

O banco de dados SQLite (`horae.db`) é criado no diretório de trabalho atual na primeira execução.
