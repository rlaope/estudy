# import 

- 정적 `import`문은 다른 모듈에서 내보낸 바인딩을 가져올 때 사용합니다.

## 구문
```js
import defaultExport from "module-name";
import * as name from "module-name";
import { export1 } from "module-name";
import { export1 as alias1 } from "module-name";
import { export1 , export2 } from "module-name";
import { foo , bar } from "module-name/path/to/specific/un-exported/file";
import { export1 , export2 as alias2 , [...] } from "module-name";
import defaultExport, { export1 [ , [...] ] } from "module-name";
import defaultExport, * as name from "module-name";
import "module-name";
let promise = import("module-name");
```

- `defalutExport` : 모듈에서 가져온 기본 내보내기를 가리킬 이름.

- `module-name` : 가져올 대상 모듈. 보통, 모듈을 담은 js파일로의 절대 또는 상대 경로

- `name` : 가져온 대상에 접근할 때 일종의 이름공간으로 사용할, 모듈 객체의 이름.

- `exportN` : 내보낸 대상 중 가져올 것의 이름.
- `aliasN` : 가져온 유명 내보내기를 가리킬 이름.

## import 설명

- 모듈 전체를 가져온다. export 한 모든 것들을 현재 범위 (스크립트 파일 하나로 구분되는 모듈 범위) 내에 `myModule`로 바인딩되어 들어갑니다.

```js
import * as myModule from "my-module.js";
```  

모듈에서 하나의 멤버만 가져옵니다. 현재 범위 내에 `myMember`이 들어갑니다.

```js
import {myMember} from "my-module.js";
```

모듈에서 여러 멤버들을 가져옵니다. 현재 범위 내에 `foo` 와 `bar`이 들어갑니다.

```js
import {foo, bar} from "my-module.js";
```