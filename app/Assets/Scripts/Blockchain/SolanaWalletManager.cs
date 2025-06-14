using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;
using Solana.Unity.SDK;
using Solana.Unity.Wallet;
using Solana.Unity.Rpc.Models;

namespace MafiaNFT.Blockchain
{
    /// <summary>
    /// Manages Solana wallet connections and transactions for the Mafia NFT game
    /// </summary>
    public class SolanaWalletManager : MonoBehaviour
    {
        [Header("Wallet Configuration")]
        [SerializeField] private string rpcUrl = "https://api.devnet.solana.com";
        [SerializeField] private string wsUrl = "wss://api.devnet.solana.com";
        [SerializeField] private bool autoConnect = true;
        
        [Header("Program IDs")]
        [SerializeField] private string characterNftProgramId = "CharNFT1111111111111111111111111111111111111";
        [SerializeField] private string mobTokenProgramId = "MOBToken111111111111111111111111111111111";
        [SerializeField] private string famTokenProgramId = "FAMToken111111111111111111111111111111111";
        [SerializeField] private string turfControlProgramId = "TurfCtrl111111111111111111111111111111111";
        
        // Events
        public static event Action<Account> OnWalletConnected;
        public static event Action OnWalletDisconnected;
        public static event Action<string> OnTransactionConfirmed;
        public static event Action<string> OnTransactionFailed;
        public static event Action<decimal> OnSolBalanceUpdated;
        public static event Action<decimal> OnMobBalanceUpdated;
        public static event Action<decimal> OnFamBalanceUpdated;
        
        // Properties
        public static SolanaWalletManager Instance { get; private set; }
        public bool IsConnected => Web3.Account != null;
        public Account CurrentAccount => Web3.Account;
        public string WalletAddress => IsConnected ? CurrentAccount.PublicKey.Key : string.Empty;
        
        // Private fields
        private Coroutine balanceUpdateCoroutine;
        private readonly Dictionary<string, decimal> tokenBalances = new Dictionary<string, decimal>();
        
        private void Awake()
        {
            if (Instance == null)
            {
                Instance = this;
                DontDestroyOnLoad(gameObject);
                InitializeWallet();
            }
            else
            {
                Destroy(gameObject);
            }
        }
        
        private void Start()
        {
            if (autoConnect)
            {
                StartCoroutine(AutoConnectWallet());
            }
        }
        
        private void OnDestroy()
        {
            if (balanceUpdateCoroutine != null)
            {
                StopCoroutine(balanceUpdateCoroutine);
            }
        }
        
        /// <summary>
        /// Initialize the Solana Web3 connection
        /// </summary>
        private void InitializeWallet()
        {
            try
            {
                Web3.OnLogin += OnWalletLogin;
                Web3.OnLogout += OnWalletLogout;
                
                Debug.Log($"[SolanaWalletManager] Initialized with RPC: {rpcUrl}");
            }
            catch (Exception ex)
            {
                Debug.LogError($"[SolanaWalletManager] Failed to initialize: {ex.Message}");
            }
        }
        
        /// <summary>
        /// Attempt to auto-connect to a previously connected wallet
        /// </summary>
        private IEnumerator AutoConnectWallet()
        {
            yield return new WaitForSeconds(1f);
            
            if (!IsConnected)
            {
                Debug.Log("[SolanaWalletManager] Attempting auto-connect...");
                // Try to restore previous session
                // This would depend on the specific wallet adapter implementation
            }
        }
        
        /// <summary>
        /// Connect to a Solana wallet
        /// </summary>
        public async Task<bool> ConnectWallet()
        {
            try
            {
                Debug.Log("[SolanaWalletManager] Connecting to wallet...");
                
                await Web3.Instance.LoginWalletAdapter();
                
                if (IsConnected)
                {
                    Debug.Log($"[SolanaWalletManager] Connected to wallet: {WalletAddress}");
                    StartBalanceUpdates();
                    return true;
                }
                
                return false;
            }
            catch (Exception ex)
            {
                Debug.LogError($"[SolanaWalletManager] Failed to connect wallet: {ex.Message}");
                return false;
            }
        }
        
        /// <summary>
        /// Disconnect from the current wallet
        /// </summary>
        public async Task DisconnectWallet()
        {
            try
            {
                if (IsConnected)
                {
                    await Web3.Instance.Logout();
                    StopBalanceUpdates();
                    Debug.Log("[SolanaWalletManager] Wallet disconnected");
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"[SolanaWalletManager] Failed to disconnect wallet: {ex.Message}");
            }
        }
        
        /// <summary>
        /// Get the current SOL balance
        /// </summary>
        public async Task<decimal> GetSolBalance()
        {
            if (!IsConnected) return 0;
            
            try
            {
                var balance = await Web3.Instance.WalletBase.GetBalance();
                var solBalance = (decimal)balance / 1_000_000_000; // Convert lamports to SOL
                
                OnSolBalanceUpdated?.Invoke(solBalance);
                return solBalance;
            }
            catch (Exception ex)
            {
                Debug.LogError($"[SolanaWalletManager] Failed to get SOL balance: {ex.Message}");
                return 0;
            }
        }
        
        /// <summary>
        /// Get token balance for a specific mint
        /// </summary>
        public async Task<decimal> GetTokenBalance(string mintAddress, int decimals = 9)
        {
            if (!IsConnected) return 0;
            
            try
            {
                var tokenAccounts = await Web3.Instance.WalletBase.GetTokenAccounts();
                
                foreach (var account in tokenAccounts)
                {
                    if (account.Account.Data.Parsed.Info.Mint == mintAddress)
                    {
                        var balance = decimal.Parse(account.Account.Data.Parsed.Info.TokenAmount.Amount);
                        var adjustedBalance = balance / (decimal)Math.Pow(10, decimals);
                        
                        tokenBalances[mintAddress] = adjustedBalance;
                        
                        // Trigger specific events based on token type
                        if (mintAddress == mobTokenProgramId)
                        {
                            OnMobBalanceUpdated?.Invoke(adjustedBalance);
                        }
                        else if (mintAddress == famTokenProgramId)
                        {
                            OnFamBalanceUpdated?.Invoke(adjustedBalance);
                        }
                        
                        return adjustedBalance;
                    }
                }
                
                return 0;
            }
            catch (Exception ex)
            {
                Debug.LogError($"[SolanaWalletManager] Failed to get token balance: {ex.Message}");
                return 0;
            }
        }
        
        /// <summary>
        /// Send a transaction and wait for confirmation
        /// </summary>
        public async Task<string> SendTransaction(Transaction transaction)
        {
            if (!IsConnected)
            {
                throw new InvalidOperationException("Wallet not connected");
            }
            
            try
            {
                Debug.Log("[SolanaWalletManager] Sending transaction...");
                
                var result = await Web3.Instance.WalletBase.SignAndSendTransaction(transaction);
                
                if (!string.IsNullOrEmpty(result))
                {
                    Debug.Log($"[SolanaWalletManager] Transaction sent: {result}");
                    
                    // Wait for confirmation
                    await WaitForTransactionConfirmation(result);
                    
                    OnTransactionConfirmed?.Invoke(result);
                    return result;
                }
                
                throw new Exception("Transaction failed to send");
            }
            catch (Exception ex)
            {
                Debug.LogError($"[SolanaWalletManager] Transaction failed: {ex.Message}");
                OnTransactionFailed?.Invoke(ex.Message);
                throw;
            }
        }
        
        /// <summary>
        /// Wait for transaction confirmation
        /// </summary>
        private async Task WaitForTransactionConfirmation(string signature)
        {
            const int maxRetries = 30;
            const int delayMs = 2000;
            
            for (int i = 0; i < maxRetries; i++)
            {
                try
                {
                    var status = await Web3.Instance.WalletBase.GetTransaction(signature);
                    
                    if (status != null)
                    {
                        Debug.Log($"[SolanaWalletManager] Transaction confirmed: {signature}");
                        return;
                    }
                }
                catch
                {
                    // Transaction not yet confirmed, continue waiting
                }
                
                await Task.Delay(delayMs);
            }
            
            throw new TimeoutException($"Transaction confirmation timeout: {signature}");
        }
        
        /// <summary>
        /// Start periodic balance updates
        /// </summary>
        private void StartBalanceUpdates()
        {
            if (balanceUpdateCoroutine == null)
            {
                balanceUpdateCoroutine = StartCoroutine(UpdateBalancesPeriodically());
            }
        }
        
        /// <summary>
        /// Stop periodic balance updates
        /// </summary>
        private void StopBalanceUpdates()
        {
            if (balanceUpdateCoroutine != null)
            {
                StopCoroutine(balanceUpdateCoroutine);
                balanceUpdateCoroutine = null;
            }
        }
        
        /// <summary>
        /// Coroutine to update balances periodically
        /// </summary>
        private IEnumerator UpdateBalancesPeriodically()
        {
            while (IsConnected)
            {
                yield return StartCoroutine(UpdateAllBalances());
                yield return new WaitForSeconds(30f); // Update every 30 seconds
            }
        }
        
        /// <summary>
        /// Update all token balances
        /// </summary>
        private IEnumerator UpdateAllBalances()
        {
            var solTask = GetSolBalance();
            var mobTask = GetTokenBalance(mobTokenProgramId);
            var famTask = GetTokenBalance(famTokenProgramId);
            
            yield return new WaitUntil(() => solTask.IsCompleted && mobTask.IsCompleted && famTask.IsCompleted);
        }
        
        /// <summary>
        /// Handle wallet login event
        /// </summary>
        private void OnWalletLogin(Account account)
        {
            Debug.Log($"[SolanaWalletManager] Wallet logged in: {account.PublicKey.Key}");
            OnWalletConnected?.Invoke(account);
            StartBalanceUpdates();
        }
        
        /// <summary>
        /// Handle wallet logout event
        /// </summary>
        private void OnWalletLogout()
        {
            Debug.Log("[SolanaWalletManager] Wallet logged out");
            StopBalanceUpdates();
            tokenBalances.Clear();
            OnWalletDisconnected?.Invoke();
        }
        
        /// <summary>
        /// Get cached token balance
        /// </summary>
        public decimal GetCachedTokenBalance(string mintAddress)
        {
            return tokenBalances.TryGetValue(mintAddress, out var balance) ? balance : 0;
        }
        
        /// <summary>
        /// Check if wallet has sufficient balance for transaction
        /// </summary>
        public async Task<bool> HasSufficientBalance(string tokenMint, decimal requiredAmount)
        {
            if (tokenMint == "SOL")
            {
                var solBalance = await GetSolBalance();
                return solBalance >= requiredAmount;
            }
            
            var tokenBalance = await GetTokenBalance(tokenMint);
            return tokenBalance >= requiredAmount;
        }
    }
}
