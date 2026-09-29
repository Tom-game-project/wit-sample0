import { $init, userFuncs } from "./pkg/webtransport_wit"

if ($init) {
  $init.then(() => {
    const result = userFuncs.helloWorld("Tom");
    console.log(result);
  });
}

