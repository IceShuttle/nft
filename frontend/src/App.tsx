import { UseInkProvider } from 'useink';
import { RococoContractsTestnet, ShibuyaTestnet } from 'useink/chains';
import {ConnectWallet} from './Wallet.tsx';

function App() {
  return (
    <UseInkProvider 
      config={{ 
        dappName: 'NFT', 
        chains: [RococoContractsTestnet, ShibuyaTestnet] ,
      }}
    >
    <ConnectWallet />
    <MyRoutes />
    </UseInkProvider>
  )
}

export default App
