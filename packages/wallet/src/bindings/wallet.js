export async function ffi_connect(cosmjs, networkConfig, chainEnv, seedPhrase) {
    const getSigner = async (config) => {
        const {chain_id, addr_prefix} = config;

        if(seedPhrase && seedPhrase !== "") {
            return await cosmjs.DirectSecp256k1HdWallet.fromMnemonic(
                seedPhrase, 
                { 
                    prefix: addr_prefix,
                }
            );
        } else {
            if (!window.keplr) {
                throw new Error("Please install keplr extension");
            }
            await window.keplr.enable(chain_id);
            return await window.getOfflineSigner(chain_id);
        }
    }
    const connectSigning = async (config) => {
        const {
            rpc_url,
            chain_id,
            denom,
            addr_prefix,
            gas_price,
            full_denom
        } = config;

        const signer = await getSigner(config);

        const client = await cosmjs.SigningCosmWasmClient.connectWithSigner(
            rpc_url,
            signer,
            { 
                gasPrice: cosmjs.GasPrice.fromString(`${gas_price}${denom}`), 
            }
        );

        const accounts = await client.signer.getAccounts();
        if(!accounts || !accounts.length) {
            throw new Error(`gotta get some funds first!`);
        } else {
            const address = accounts[0].address;
            return {signer: client.signer, address, client, ...config, env: chainEnv}
        }
    }

    const neutron = await connectSigning(chainEnv === "local" ? networkConfig.neutron_local : networkConfig.neutron_testnet);
    const kujira = await connectSigning(chainEnv === "local" ? networkConfig.kujira_local : networkConfig.kujira_testnet);
    const stargaze = await connectSigning(chainEnv === "local" ? networkConfig.stargaze_local : networkConfig.stargaze_testnet);
    const nois = await connectSigning(chainEnv === "local" ? networkConfig.nois_local : networkConfig.nois_testnet);

    return {
        neutron: {
            ...neutron,
            networkId: "neutron",
        },
        kujira: {
            ...kujira,
            networkId: "kujira",
        },
        stargaze: {
            ...stargaze, 
            networkId: "stargaze",
        },
        nois: {
            ...nois, 
            networkId: "nois",
        },
    }
}

export async function ffi_install_keplr(networkConfig, chainEnv) {
    if (!window.keplr) {
        alert("Please install keplr extension");
        return;
    }

    async function installKeplr(config) {
        const currency = {
            coinDenom: config.denom,
            coinMinimalDenom: config.denom,
            coinDecimals: 6,
            coinGeckoId: config.full_denom,
        }

        const keplrConfig = {
            chainId:  config.chain_id,
            chainName: config.chain_id,
            rpc: config.rpc_url,
            rest: config.rest_url, 
            bip44: {
                coinType: 118,
            },
            bech32Config: {
                bech32PrefixAccAddr: config.addr_prefix,
                bech32PrefixAccPub: `${config.addr_prefix}pub`,
                bech32PrefixValAddr: `${config.addr_prefix}valoper`,
                bech32PrefixValPub: `${config.addr_prefix}valoperpub`,
                bech32PrefixConsAddr: `${config.addr_prefix}valcons`,
                bech32PrefixConsPub: `${config.addr_prefix}valconspub`
            },
            currencies: [currency],
            feeCurrencies: [currency],
            stakeCurrency: currency,
        }

        await window.keplr.experimentalSuggestChain(keplrConfig)
    }

    await installKeplr(chainEnv === "local" ? networkConfig.neutron_local : networkConfig.neutron_testnet);
    await installKeplr(chainEnv === "local" ? networkConfig.kujira_local : networkConfig.kujira_testnet);
    await installKeplr(chainEnv === "local" ? networkConfig.stargaze_local : networkConfig.stargaze_testnet);
    await installKeplr(chainEnv === "local" ? networkConfig.nois_local : networkConfig.nois_testnet);
}

export async function ffi_contract_query(wallet, contractAddress, msg) {
    return await wallet.client.queryContractSmart(contractAddress, msg);
}

export async function ffi_contract_code_details(wallet, codeId) {
    return await wallet.client.getCodeDetails(codeId);
}

export async function ffi_contract_info(wallet, addr) {
    return await wallet.client.getContract(addr);
}

export async function ffi_contract_upload(wallet, data) {
    return await wallet.client.upload(wallet.address, data, "auto");
}

export async function ffi_contract_instantiate(wallet, codeId, msg, label) {
    return await wallet.client.instantiate(wallet.address, codeId, msg, label, "auto", { admin: wallet.address });
}

export async function ffi_contract_migrate(wallet, addr, codeId, msg) {
    return await wallet.client.migrate(wallet.address, addr, codeId, msg, "auto");
}

export async function ffi_contract_exec(wallet, contractAddress, msg) {
    return await wallet.client.execute(wallet.address, contractAddress, msg, "auto", "");
} 

export async function ffi_contract_exec_funds(wallet, contractAddress, msg, funds) {
    try {
        return await wallet.client.execute(wallet.address, contractAddress, msg, "auto", "", funds);
    } catch(e) {
        console.error(e);
    }
} 

export async function ffi_wallet_balance(wallet) {
    const coin = await wallet.client.getBalance(wallet.address, wallet.denom);
    return Number(coin.amount)
}