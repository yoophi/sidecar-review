---
name: sidecar-review
description: Show the human reviewer what a coding session changed, live in the Sidecar Review desktop app, with the sidecar MCP tool push_review. Use after each meaningful unit of work when the sidecar MCP server is connected, and whenever the user says "sidecar review", "sidecar로 표시", "리뷰를 sidecar에 올려" or asks to see changes, core code, diffs, charts or before/after screenshots in Sidecar.
---

# Sidecar Review

코딩 세션에서 바뀐 핵심을 `push_review`(sidecar MCP)로 보내면 Sidecar Review 앱에 바로 뜬다. 리뷰어는 사이드바에서 세션을 고르고, 파일 트리에서 파일을 눌러 그 파일의 블록만 본다.

## 언제 보내나

- 의미 있는 작업 단위를 끝낼 때마다 한 번씩 보낸다. 마지막에 몰아서 보내지 않는다.
- 사용자가 "sidecar review", "sidecar로 표시"처럼 요청하면 그때까지의 변경을 정리해 보낸다.

## 세션

- `session`은 작업 내내 같은 값을 쓴다. 브랜치나 작업 이름이 좋다(`feat/login-retry`). 같은 `session`으로 보내면 블록이 뒤에 이어 붙는다.
- `goal`에는 작업의 목표를 사용자 언어로 한 줄 적는다. 리뷰 맨 위 제목으로 나온다. 바꾼 것을 나열하지 말고("파일 트리 + 배지 추가") 그 작업으로 이루려는 것을 쓴다("리뷰어가 파일 단위로 변경을 찾아볼 수 있게 한다"). 보낼 때마다 마지막 값으로 바뀌므로, 세션 전체의 목표를 매번 같은 문장으로 보낸다.
- 작업 순서대로 쌓인 세션이 길어졌고 사용자가 정리본을 원하면, 새 `session`에 최종 코드 기준으로 다시 올린다.

- `agent`(claude, opencode, codex 같은 에이전트 이름), `cwd`(작업 디렉터리 절대 경로), `agent_session_id`(에이전트 자신의 대화 ID, 알 때만)를 매번 함께 보낸다. 세션 목록의 ⓘ 아이콘에 나온다.

## 블록

핵심만 보낸다. 로직을 담은 몇 줄만 남기고 import, 보일러플레이트, 포맷 변경, 관련 없는 주변 코드는 뺀다. 설명 문단은 쓰지 않고, `title`은 그 블록이 다루는 함수나 동작을 한 줄로 적는다.

| type | 필드 | 쓰는 때 |
|---|---|---|
| `diff` | `file`, `before`, `after`, `mermaid` | 고친 코드. 새 코드면 `before`를, 지운 코드면 `after`를 뺀다 |
| `code` | `file`, `code`, `mermaid` | 새 파일의 핵심, 또는 바뀌지 않았지만 리뷰에 필요한 코드 |
| `mermaid` | `source` | 여러 파일에 걸친 전체 흐름 |
| `html` | `html` | 차트, 화면 before/after 스크린샷 |

- `file`은 저장소 기준 상대 경로로, 같은 파일이면 블록마다 똑같이 쓴다. 앱이 이 값으로 디렉터리 트리를 만들고 블록을 묶는다.
- `before`와 `after`는 같은 구간을 잘라 온다. 구간이 어긋나면 diff가 엉뚱한 줄을 가리킨다.

## 차트 규칙

모든 `diff`·`code` 블록에 그 코드를 가장 잘 설명하는 Mermaid 차트를 `mermaid` 필드로 붙인다. 차트는 같은 블록 안에서 코드 위에 그려진다. 필드 한 줄 추가나 문구 수정처럼 차트 없이도 명백한 경우에만 뺀다.

- 분기와 데이터 흐름은 `flowchart`, 주체 사이에 오가는 호출은 `sequenceDiagram`, 상태 변화는 `stateDiagram-v2`.
- 코드를 그대로 옮기지 말고, 코드만 봐서는 바로 안 보이는 것(분기 조건, 호출 순서, 누가 상태를 갖는지)을 그린다.
- 괄호, 따옴표, 콜론이 들어간 라벨은 `["..."]`로 감싼다. 문법이 틀리면 차트 자리에 오류 문구가 나온다.

## HTML 블록

- 자체 완결 HTML이어야 한다. sandbox iframe 안에서 돌고 높이는 420px 고정이다.
- 스크린샷은 `data:` URI로 넣는다. 본문이 커서 도구 호출로 보내기 어려우면 같은 엔드포인트에 JSON-RPC로 직접 POST한다(요청 하나에 약 2MB까지). `Origin` 헤더가 붙으면 서버가 거부한다.

```sh
curl -s http://127.0.0.1:7437/mcp -H 'Content-Type: application/json' -d @request.json
# request.json: {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"push_review","arguments":{...}}}
```

## 지킬 것

- 비밀값, 토큰, 개인정보가 든 코드는 보내지 않는다.
- 바뀌지 않은 것을 바뀐 것처럼 꾸미지 않는다. 변경이 없으면 `title`에 "변경 없음"이라고 쓴다.
- `push_review`가 없거나 실패하면 앱이 꺼져 있는 것이다. 사용자에게 앱을 켜 달라고 알린다. 연결 명령: `claude mcp add --transport http sidecar http://127.0.0.1:7437/mcp`

## 예

```json
{
  "session": "feat/login-retry",
  "goal": "로그인 실패 시 재시도 횟수 제한",
  "blocks": [
    {
      "type": "diff",
      "title": "실패 횟수 확인 추가",
      "file": "src/auth.ts",
      "before": "return verify(user, pw);",
      "after": "if (user.tries > 5) throw new Locked();\nreturn verify(user, pw);",
      "mermaid": "flowchart TD\n  L[login] --> Q{\"tries > 5\"}\n  Q -- 예 --> X[Locked]\n  Q -- 아니오 --> V[verify]"
    }
  ]
}
```
