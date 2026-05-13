# Дизайн-спецификация: Agent Skill для `hd-cli`

Дата: 2026-05-13  
Проект-источник CLI: `nimblemo/Human-Design-cli`  
Цель: создать отдельный репозиторий со Skill, который (1) скачивает и кэширует последний релиз бинарника `hd-cli` под платформу клиента и (2) даёт агентам удобный интерфейс запуска CLI в structured и raw режимах.

## 1. Область и допущения

### 1.1. Что делаем
- Публикуем релиз-ассеты `hd-cli` в GitHub Releases репозитория `nimblemo/Human-Design-cli` для Linux/macOS/Windows x86_64.
- Создаём отдельный репозиторий Skill (далее `hd-cli-agent-skill`) со следующими возможностями:
  - Установка: определить ОС/архитектуру, скачать “latest release” ассет, проверить контрольную сумму (если доступна), положить в кэш.
  - Запуск: дать агентам две модели использования:
    - structured: задать поля (`date/time/utc/lang/...`) и вернуть структурированный результат (по умолчанию JSON).
    - raw: проксировать произвольные аргументы в бинарник и вернуть stdout/stderr.
  - Fallback при невозможности скачать релиз: сообщить пользователю причину и предложить локальную установку из репозитория (сборка через cargo).

### 1.2. Что не делаем
- Не публикуем в PATH глобально (по умолчанию) и не меняем системную конфигурацию клиента.
- Не храним секреты и токены в репозитории/логах.
- Не пытаемся обойти ограничения доступа к GitHub; при 403/404/timeout корректно сообщаем и предлагаем локальную установку.

## 2. Требования к релизам `nimblemo/Human-Design-cli`

### 2.1. Триггеры релиза
- Релиз создаётся при push тега вида `vX.Y.Z`.
- Версия бинарника должна соответствовать тэгу.

### 2.2. Матрица сборки
- linux-x86_64 (GNU)
- darwin-x86_64
- windows-x86_64

### 2.3. Имена ассетов (контракт)
Установщик в Skill рассчитывает имя ассета детерминированно, поэтому имена должны быть стабильны.

- Linux: `hd-cli-v{version}-linux-x86_64.tar.gz`
- macOS: `hd-cli-v{version}-darwin-x86_64.tar.gz`
- Windows: `hd-cli-v{version}-windows-x86_64.zip`
- Контрольные суммы: `SHA256SUMS`

### 2.4. Содержимое архивов
- Linux/macOS tar.gz содержит исполняемый файл `hd-cli`
- Windows zip содержит `hd-cli.exe`

### 2.5. Контроль целостности
- `SHA256SUMS` содержит строки формата:
  - `<sha256>  <asset-filename>`
- Установщик:
  - Если файл `SHA256SUMS` найден в релизе — сверяет sha256 скачанного ассета.
  - Если отсутствует — продолжает без проверки, но сообщает пользователю, что верификация недоступна.

## 3. Skill-репозиторий: структура и артефакты

### 3.1. Репозиторий
Предлагаемое имя: `hd-cli-agent-skill` (отдельный GitHub repo).

### 3.2. Дерево файлов
```
hd-cli-agent-skill/
  skills/
    hd-cli/
      SKILL.md
      .local/
        calculations.jsonl
        people/
          <person-id>.json
        charts/
          <chart-id>.json
        dialogs/
          <dialog-id>.json
        artifacts/
          <artifact-id>.json
        notebooklm/
          sources.json
      scripts/
        install.sh
        install.ps1
        github_release.py
        platform.py
        run.py
      templates/
        error_download.md
        local_install.md
  README.md
  LICENSE
```

### 3.3. Путь установки/кэширование
- Cache root: `~/.cache/hd-cli/`
- Layout:
  - `~/.cache/hd-cli/{version}/hd-cli` (Linux/macOS)
  - `~/.cache/hd-cli/{version}/hd-cli.exe` (Windows)
- Симлинк/алиас на текущую версию:
  - `~/.cache/hd-cli/current/hd-cli[.exe]` (копия или симлинк, зависит от платформы)

### 3.4. Персистентное хранение (локально в директории скилла)
Требование: хранить данные персистентно локально в директории скилла.

- Local state root: `<skill_root>/skills/hd-cli/.local/`
- Данные хранятся в JSON/JSONL, чтобы их можно было бэкапить, переносить и инспектировать.
- Все записи содержат:
  - `id` (строка UUID/slug)
  - `created_at` (ISO 8601)
  - `updated_at` (ISO 8601)

Файлы:
- `calculations.jsonl` — журнал всех расчётов (append-only).
- `people/<person-id>.json` — сущность “человек”.
- `charts/<chart-id>.json` — сохранённая “карта” (compact-версия без описательных полей).
- `dialogs/<dialog-id>.json` — диалог, связанный с 1+ людьми/картами.
- `artifacts/<artifact-id>.json` — артефакт диалога: вопрос пользователя + прикреплённые данные карт + ответ NotebookLM.
- `notebooklm/sources.json` — маппинг локальных сущностей на NotebookLM sources (source-id, заголовок, контрольная сумма контента, время синхронизации).

## 4. Установщик: алгоритм

### 4.1. Определение платформы
- OS: linux | darwin | windows
- arch: x86_64
- Platform key: `{os}-{arch}`

### 4.2. Получение latest release
Источник: GitHub API:
- `GET https://api.github.com/repos/nimblemo/Human-Design-cli/releases/latest`

Логика:
- Если 200: распарсить `tag_name`, список `assets[]`, найти ассет по имени.
- Если 404: релизов нет → fallback (локальная установка).
- Если 403/401: нет доступа/лимиты → fallback (локальная установка).
- Если сеть недоступна/timeout: fallback (локальная установка).

### 4.3. Скачивание ассета
- Скачать `browser_download_url` ассета в временный файл.
- (Опционально) скачать `SHA256SUMS` и проверить sha256.
- Распаковать в `{cache_root}/{version}/`.
- Выставить executable bit (Linux/macOS).
- Обновить `current/`.

### 4.4. Поведение при ошибках
Если релиз нельзя скачать или ассет не найден:
- Вернуть понятную ошибку:
  - причина (нет релиза / нет доступа / нет ассета под платформу / сеть)
  - следующий шаг: “локальная установка”
- Предложить пользователю одну из команд:
  - `git clone https://github.com/nimblemo/Human-Design-cli`
  - `cargo build --release` или `cargo install --path .`
- Если пользователь запускает скилл повторно с флагом вроде `--from-source <path>` (опционально в реализации), скилл может собрать бинарник из указанного пути.

## 5. Интерфейс Skill для агентов

### 5.1. Поддерживаемые режимы
- `structured` режим:
  - Вход: поля даты/времени/UTC/языка и флаги (`short`, `format`).
  - Исполнение: вызвать установленный бинарник `hd-cli` с `--format json` (по умолчанию).
  - Выход: вернуть JSON (stdout) и нормализованный статус.
- `raw` режим:
  - Вход: строка аргументов, которые нужно передать `hd-cli` как есть.
  - Выход: stdout/stderr + exit code.

### 5.2. Сигнатура аргументов (proposal)
- `--mode structured|raw` (default: structured)
- Structured:
  - `--date YYYY-MM-DD`
  - `--time HH:MM`
  - `--utc +3|-5|+5.5`
  - `--lang ru|en|es` (default: ru)
  - `--short`
  - `--format json|yaml|table` (default: json)
- Raw:
  - `--args "<raw args>"`

### 5.3. Контракты вывода
- structured:
  - если exit code != 0: вернуть ошибку + stderr
  - если `--format json`: попытаться распарсить JSON; при невалидном JSON — вернуть raw stdout + ошибку парсинга
- raw:
  - всегда вернуть `exit_code`, `stdout`, `stderr`

### 5.4. Данные карт без текстовых описаний (compact chart)
Требование: при прикреплении данных карт к NotebookLM не включать текстовые описания из расчётов.

Источник данных: JSON вывод `hd-cli --format json` (см. модель [HdChart](file:///workspace/src/models.rs#L54-L99)).

Правило “compact”:
- Сохранять только структурные поля и идентификаторы.
- Удалять/не сохранять следующие поля (если присутствуют):
  - `*_description` (например, `type_description`, `profile_description`, `authority_description`, `strategy_description`, `cross_description`)
  - `personality[].gate_name`, `personality[].gate_description`, `personality[].line_description`
  - `design[].gate_name`, `design[].gate_description`, `design[].line_description`
  - `channels[].description`
  - `centers[].behavior_normal`, `centers[].behavior_distorted`
  - любые `InfoItem.description` и подобные блоки, если они добавляются в JSON

Хранимый JSON “compact chart” должен включать (минимум):
- `birth_date`, `birth_time`, `utc_offset`
- `type`, `profile`, `authority`, `strategy`, `incarnation_cross`
- `personality[]`: `planet`, `index`, `longitude`, `degree`, `zodiac_sign`, `gate`, `line`, `color`, `tone`, `base`
- `design[]`: то же
- `channels[]`: `key`, `name`
- `centers[]`: `name`, `defined`

## 6. Интеграция с NotebookLM через `notebooklm-cli`

### 6.1. Зависимость от `nlm-cli-skill`
Требование: система зависит от скилла `nlm-cli-skill` и использует его правила/ограничения.

Источник: [nlm-cli-skill/SKILL.md](https://raw.githubusercontent.com/jacob-bd/notebooklm-cli/main/nlm-cli-skill/SKILL.md)

Ключевые правила интеграции:
- Перед любыми операциями проверять аутентификацию (`nlm login --check`), иначе просить пользователя выполнить `nlm login`.
- Не использовать `nlm chat start` (интерактивный REPL). Только `nlm notebook query`.
- Все запросы выполняются в фиксированном NotebookLM ноутбуке:
  - `notebook_id = c5dd30c7-da41-49a5-a0ce-74ed7ad7ce1b`
  - URL: `https://notebooklm.google.com/notebook/c5dd30c7-da41-49a5-a0ce-74ed7ad7ce1b`

### 6.2. Привязка людей/карт к NotebookLM (через sources)
Выбранная стратегия: “через source”.

- Для каждого `person-id` поддерживать один NotebookLM source типа `--text` с title вида:
  - `HD Chart: <display_name> (<person-id>)`
- Контент source: JSON “compact chart” (см. раздел 5.4), возможно упакованный в один объект:
  - `{ "person": {...}, "chart": {...}, "chart_id": "...", "calculation": {...} }`
- Локально сохранять маппинг в `.local/notebooklm/sources.json`:
  - `person_id -> { source_id, title, content_sha256, updated_at }`

### 6.3. Диалоги и артефакты (локально)
Сущности:
- Person:
  - `id`, `display_name`, (опционально) заметки
  - список связанных `chart_id` (история)
- Calculation (журнал, JSONL):
  - `calculation_id`, `person_id`, входные параметры (`date/time/utc/lang`), `chart_id`, `hd_cli_version`, `created_at`
- Dialog:
  - `id`, `title`, `participant_person_ids[]`, `notebook_id` (фиксированный), `conversation_id` (если используется для follow-up), `artifact_ids[]`
- Artifact:
  - `id`, `dialog_id`, `created_at`
  - `user_query` (текст пользователя)
  - `attached_people[]`: массив `{ person_id, chart_id, chart_compact }`
  - `notebooklm`: `{ notebook_id, conversation_id?, request_text, response_text, status }`

Поведение:
- При новом вопросе в рамках диалога:
  1) Убедиться, что все участники имеют актуальные sources в NotebookLM (создать/обновить при необходимости).
  2) Выполнить `nlm notebook query <notebook-id> "..."`:
     - Если у диалога уже есть `conversation_id` — передать его флагом `--conversation-id`.
     - Если `conversation_id` ещё нет — сохранить `conversation_id` из ответа (если CLI его возвращает), чтобы продолжать контекстно.
  3) Сохранить Artifact локально, включая снимок `chart_compact` для каждого участника (чтобы артефакт был самодостаточным).

## 7. Безопасность и приватность
- Не логировать токены/credential’ы.
- При запросах к GitHub API использовать публичный доступ без токена; если пользователь настроил токен локально — допустимо поддержать через env var, но не требовать.
- Не записывать скачанные файлы вне cache root.

## 8. Критерии приемки
- При наличии GitHub Release с ассетом под платформу:
  - Skill скачивает latest release, кладёт в кэш, запускает `hd-cli`, возвращает результат.
- При отсутствии релизов или недоступности GitHub API:
  - Skill сообщает причину и предлагает локальную установку из `nimblemo/Human-Design-cli`.
- Поддержаны оба режима (structured + raw).
- Персистентно сохраняются:
  - список расчётов (calculations.jsonl)
  - люди/карты
  - диалоги и артефакты диалогов с прикреплёнными compact-данными карт
- NotebookLM интеграция:
  - источники (sources) создаются/обновляются на человека
  - запросы выполняются через `nlm notebook query` в указанном notebook_id
- Нет секретов в логах/файлах.
