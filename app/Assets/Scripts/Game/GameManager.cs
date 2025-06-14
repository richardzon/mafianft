using UnityEngine;

namespace MafiaNFT.Game
{
    public class GameManager : MonoBehaviour
    {
        public static GameManager Instance { get; private set; }
        
        [Header("Game Configuration")]
        [SerializeField] private bool autoConnectWallet = true;
        [SerializeField] private string gameVersion = "1.0.0";
        
        public bool IsGameInitialized { get; private set; }
        
        private void Awake()
        {
            if (Instance == null)
            {
                Instance = this;
                DontDestroyOnLoad(gameObject);
                InitializeGame();
            }
            else
            {
                Destroy(gameObject);
            }
        }
        
        private void InitializeGame()
        {
            Debug.Log("Initializing Mafia NFT Game v" + gameVersion);
            IsGameInitialized = true;
        }
        
        public void StartGame()
        {
            Debug.Log("Starting game...");
        }
        
        public void QuitGame()
        {
            Debug.Log("Quitting game...");
            Application.Quit();
        }
    }
}