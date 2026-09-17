# Stateful, Stateless

### Stateless Protocol
- The server does not retain the client's state.

#### Advantages
High server scalability (scale out)

#### Disadvantages
Client sends additional data

#### ex)

Stateful
```
고객: 이 노트북 얼마인가요?
점원: 100만원 입니다. (노트북 상태 유지)
고객: 2개 구매하겠습니다.
점원: 200만원 입니다. 신용카드, 현금 중에 어떤 걸로 구매 하시겠어요? (노트북, 2개 상태 유지)
고객: 신용카드로 구매하겠습니다.
점원: 200만원 결제 완료되었습니다. (노트북, 2개, 신용카드 상태 유지)
```

Stateless
```
고객: 이 노트북 얼마인가요?
점원: 100만원 입니다.
고객: 노트북 2개 구매하겠습니다.
점원: 노트북 2개는 200만원 입니다. 신용카드, 현금중에 어떤 걸로 구매 하시겠어요?
고객: 노트북 2개를 신용카드로 구매하겠습니다.
점원: 200만원 결제 완료되었습니다.
```

#### Stateful, Stateless Differences

- Stateful: Cannot switch to a different clerk in the middle >> If the server fails in the middle, the client's work must be restarted from the beginning. (When switching to a different clerk, state information must be communicated to the new clerk in advance.)
- Stateless: Can switch to a different clerk in the middle.
  - Can deploy many clerks even if customer numbers suddenly increase.
  - Can deploy many servers even if client requests suddenly increase (scale out).
- Stateless allows for easy switching of response servers -> infinite server expansion is possible.

<br>

### Stateless Practical Limitations
- There are cases where everything can be designed as stateless, and cases where it cannot.
- Stateless example: A simple service introduction screen that doesn't require login.
- Stateful example: Login.
- For logged-in users, the login state is maintained on the server.
- Typically, browser cookies and server sessions are used to maintain state.
- Use statefulness only when absolutely necessary.
