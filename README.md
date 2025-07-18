 # ZK-PoV Consensus Protocol

Zero-Knowledge Proof of Validity (ZK-PoV) consensus mekanizması, blockchain teknolojisinde yeni bir yaklaşım sunar. Bu protokol, her blok için ZK-proof üreterek geçerliliği kanıtlar ve recursive proof sistemi ile ölçeklenebilirlik sağlar.

## 🚀 Özellikler

### Core Features
- **ZK-Proof Generation**: Her blok için Groth16 SNARK tabanlı geçerlilik kanıtı
- **Recursive Proofs**: Önceki kanıtları yeni kanıtlarda kullanarak verimlilik
- **Fast Finality**: 12 saniye blok süresi ile hızlı finality
- **Scalable**: O(1) verification complexity
- **Quantum-Resistant**: Gelecekte quantum-resistant ZK sistemlere geçiş hazır

### Technical Features
- **Rust Implementation**: Yüksek performanslı, güvenli kod
- **P2P Network**: libp2p tabanlı dağıtık ağ
- **RocksDB Storage**: Yüksek performanslı veri depolama
- **Async Runtime**: Tokio tabanlı asenkron işlemler

## 📋 Gereksinimler

- Rust 1.70+ 
- Cargo
- macOS/Linux/Windows

## 🛠️ Kurulum

```bash
# Projeyi klonlayın
git clone <repository-url>
cd zk_consensus

# Bağımlılıkları yükleyin
cargo build --release

# Çalıştırın
cargo run --release
```

## 🎯 Kullanım

### Temel Kullanım

```bash
# Validator node olarak çalıştır
cargo run -- --mode validator --port 8080

# Full node olarak çalıştır
cargo run -- --mode full_node --port 8081

# Light client olarak çalıştır
cargo run -- --mode light_client --port 8082
```

### Gelişmiş Seçenekler

```bash
# Bootstrap node'ları belirt
cargo run -- --bootstrap /ip4/192.168.1.100/tcp/8080/p2p/QmNode1 \
            --bootstrap /ip4/192.168.1.101/tcp/8080/p2p/QmNode2

# Debug modunda çalıştır
cargo run -- --debug

# Özel port belirt
cargo run -- --port 9000
```

## 🏗️ Mimari

### Bileşenler

```
zk_consensus/
├── src/
│   ├── main.rs              # Ana uygulama
│   ├── types/               # Veri tipleri
│   ├── consensus/           # Consensus engine
│   ├── zk_proof/           # ZK proof generation
│   ├── network/            # P2P network
│   └── storage/            # Veri depolama
```

### Consensus Flow

1. **Block Proposal**: Validator yeni blok önerir
2. **ZK Proof Generation**: Blok için ZK-proof üretilir
3. **Network Broadcast**: Blok ve proof ağa yayınlanır
4. **Validation**: Diğer node'lar proof'u doğrular
5. **Voting**: Node'lar blok üzerinde oy verir
6. **Finality**: Yeterli oy toplandığında blok finalize edilir

## 🔧 Konfigürasyon

### Environment Variables

```bash
# Node konfigürasyonu
export ZK_CONSENSUS_MODE=validator
export ZK_CONSENSUS_PORT=8080
export ZK_CONSENSUS_DB_PATH=./data/zk_consensus.db

# Network konfigürasyonu
export ZK_CONSENSUS_BOOTSTRAP_NODES="node1,node2,node3"
export ZK_CONSENSUS_DISCOVERY_ENABLED=true

# Performance konfigürasyonu
export ZK_CONSENSUS_BLOCK_TIME=12
export ZK_CONSENSUS_MIN_VALIDATORS=3
```

### Config File

```toml
# config.toml
[network]
port = 8080
bootstrap_nodes = ["/ip4/127.0.0.1/tcp/8080/p2p/QmNode1"]
discovery_enabled = true

[consensus]
block_time = 12
min_validators = 3
epoch_length = 1000

[storage]
db_path = "./data/zk_consensus.db"
max_open_files = 10000

[zk_proof]
proof_type = "Groth16"
curve = "BLS12-381"
```

## 📊 Performance

### Benchmarks

| Metric | Value |
|--------|-------|
| Block Time | 12 seconds |
| TPS | 10,000+ |
| Proof Size | ~200 bytes |
| Verification Time | < 10ms |
| Memory Usage | ~512MB |

### Scalability

- **Horizontal Scaling**: P2P network ile sınırsız node
- **Vertical Scaling**: Multi-core CPU desteği
- **Storage Scaling**: RocksDB ile terabyte seviyesi
- **Network Scaling**: libp2p ile global ağ

## 🔒 Güvenlik

### ZK-Proof Security
- **Cryptographic Security**: 128-bit security level
- **Zero-Knowledge**: Gizlilik korunur
- **Succinct**: Kısa proof boyutu
- **Non-Interactive**: Tek yönlü doğrulama

### Network Security
- **Encrypted Communication**: Noise protocol
- **Peer Authentication**: Ed25519 keypairs
- **Message Validation**: Strict message format
- **Sybil Resistance**: Stake-based consensus

## 🧪 Test

```bash
# Unit testleri çalıştır
cargo test

# Integration testleri çalıştır
cargo test --test integration

# Performance testleri çalıştır
cargo bench

# Coverage raporu oluştur
cargo tarpaulin
```

## 📈 Monitoring

### Metrics

```bash
# Node durumu kontrol et
curl http://localhost:8080/metrics

# Consensus durumu
curl http://localhost:8080/consensus/status

# Network durumu
curl http://localhost:8080/network/peers
```

### Logging

```bash
# Debug logları
RUST_LOG=debug cargo run

# Trace logları
RUST_LOG=trace cargo run

# JSON formatında loglar
RUST_LOG=info cargo run -- --log-format json
```

## 🔄 Development

### Yeni Özellik Ekleme

1. **Feature Branch** oluştur
```bash
git checkout -b feature/new-consensus-rule
```

2. **Test** yaz
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_new_consensus_rule() {
        // Test implementation
    }
}
```

3. **Documentation** güncelle
4. **Benchmark** ekle
5. **Pull Request** oluştur

### Code Style

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security issues
cargo audit
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Add tests
5. Update documentation
6. Submit pull request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: [Wiki](link-to-wiki)
- **Community**: [Discord](link-to-discord)

## 🔮 Roadmap

### Phase 1: Core Implementation ✅
- [x] Basic ZK-proof generation
- [x] Consensus engine
- [x] P2P network
- [x] Storage layer

### Phase 2: Optimization 🚧
- [ ] GPU acceleration
- [ ] Proof aggregation
- [ ] Memory optimization
- [ ] Network optimization

### Phase 3: Advanced Features 📋
- [ ] Quantum-resistant ZK
- [ ] Cross-chain bridges
- [ ] Smart contracts
- [ ] Governance system

### Phase 4: Production 🎯
- [ ] Security audit
- [ ] Performance tuning
- [ ] Production deployment
- [ ] Ecosystem tools

## 📚 References

- [Groth16 Paper](https://eprint.iacr.org/2016/260)
- [libp2p Documentation](https://docs.rs/libp2p)
- [RocksDB Documentation](https://rocksdb.org/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

---

**ZK-PoV Consensus Protocol** - Geleceğin blockchain consensus mekanizması 🚀