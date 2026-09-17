# import

- Static `import` statements are used to import bindings exported from other modules.

## Syntax
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

- `defaultExport` : The name to refer to the default export imported from the module.

- `module-name` : The target module to import. Typically, an absolute or relative path to the JS file containing the module.

- `name` : The name of the module object to be used as a namespace when accessing the imported target.

- `exportN` : The name of the exported target to import.
- `aliasN` : The name to refer to the imported named export.

## import Explanation

- Imports the entire module. All exported items are bound into the current scope (module scope, distinguished by a single script file) as `myModule`.

```js
import * as myModule from "my-module.js";
```  

Imports only one member from the module. `myMember` is brought into the current scope.

```js
import {myMember} from "my-module.js";
```

Imports multiple members from the module. `foo` and `bar` are brought into the current scope.

```js
import {foo, bar} from "my-module.js";
```
