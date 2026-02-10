# Antediluvia Funding & Server Strategy

**Document Version:** 1.0
**Date:** January 2026
**Purpose:** Outline funding pathways and server infrastructure costs for scaling to production

---

## 1. Server Cost Breakdown (Annual)

### Phase 1: Alpha (Current - 100 Concurrent Players)
- **Compute:** AWS EC2 t3.medium (1 server) = $30/month
- **Database:** PostgreSQL RDS db.t3.micro = $20/month
- **Storage:** S3 (asset CDN) = $10/month
- **Bandwidth:** ~50 GB/month = $5/month
- **Monitoring/Logging:** CloudWatch = $5/month
- **Total:** ~$70/month ($840/year)

### Phase 2: Beta (500 Concurrent Players)
- **Compute:** 3x EC2 t3.large (load balanced) = $90/month
- **Database:** RDS db.t3.small (multi-AZ) = $60/month
- **Cache:** ElastiCache Redis = $20/month
- **Storage/CDN:** CloudFront + S3 = $30/month
- **Bandwidth:** ~200 GB/month = $20/month
- **Monitoring:** Enhanced = $15/month
- **Total:** ~$235/month ($2,820/year)

### Phase 3: Launch (2,000+ Concurrent Players)
- **Compute:** Auto-scaling group (6-12 instances) = $300/month
- **Database:** RDS db.r5.large (read replicas) = $150/month
- **Cache/Queue:** Redis + RabbitMQ = $50/month
- **Storage/CDN:** Global CDN = $100/month
- **Bandwidth:** ~1 TB/month = $100/month
- **DDoS Protection:** AWS Shield Advanced = $30/month
- **Monitoring/Alerting:** DataDog/New Relic = $50/month
- **Total:** ~$780/month ($9,360/year)

---

## 2. Funding Pathways (Priority Order)

### Pathway A: Bootstrapped (Self-Funded)
**Timeline:** 6-12 months
**Method:** Personal savings + revenue from early access

**Milestones:**
1. Launch free tier (Alpha) on $70/month budget
2. Offer "Premium Tier" cosmetics ($4.99/month) to cover costs
3. At 100 premium players = $500/month revenue (covers Phase 2)
4. Scale to Phase 2 at 500 concurrent players

**Pros:** Full control, no dilution, aligns with "free AI for masses" vision
**Cons:** Slow growth, limited marketing budget

---

### Pathway B: Grants & Community Funding
**Timeline:** 3-6 months
**Method:** Apply for tech/gaming grants + Patreon/Ko-fi

**Targets:**
- **Epic Games MegaGrants:** $5,000-$25,000 (for Unreal/Bevy projects)
- **Mozilla Open Source Fund:** $10,000-$50,000 (Rust-based)
- **Patreon:** Target $500/month from 50 supporters at $10/month
- **Ko-fi:** One-time donations ($5-$50)

**Pros:** No equity loss, community validation, fast capital
**Cons:** Competitive, requires strong pitch, time-intensive

---

### Pathway C: Angel Investment
**Timeline:** 2-4 months
**Method:** Pitch to angel investors interested in gaming + AI

**Target Profile:**
- Investors in AI/ML infrastructure
- Gaming industry veterans
- Crypto/Web3 enthusiasts (if aligned with vision)

**Pitch Points:**
- "Distribute AI to the masses" (Deepseek + Groq backend)
- Massive TAM (MMORPG market = $20B+ annually)
- Unique "biblical realism" niche (underserved market)
- Rust/Bevy = technical moat (performance, safety)

**Funding Ask:** $100,000-$500,000 (Seed round)
**Use of Funds:**
- $50,000: Server infrastructure (Year 1)
- $30,000: Marketing & community building
- $20,000: Additional developer (contractor)

**Pros:** Large capital injection, credibility, network
**Cons:** Equity dilution (typically 10-20%), founder control loss

---

### Pathway D: Revenue-Based Financing
**Timeline:** 4-8 months
**Method:** Merchant cash advance or revenue-based financing

**Structure:**
- Borrow $50,000-$200,000 against future game revenue
- Repay 5-10% of monthly revenue until principal + interest paid
- No equity loss

**Pros:** No dilution, flexible repayment
**Cons:** Higher cost of capital, requires revenue proof

---

## 3. Revenue Model (Sustainable)

### Free Tier (Core Game)
- Unlimited access to base game
- Deepseek AI for NPC dialogue (free, with rate limits)
- Cosmetics limited to "earned" gear only

### Premium Tier ($4.99/month)
- Groq-powered "High-Fidelity" NPC conversations
- Cosmetic "Tunic of Support" (visual only, no stat advantage)
- Battle Pass (cosmetics + lore unlocks)
- No P2W mechanics

### Projected Revenue (Year 1)
- 1,000 active players
- 10% conversion to premium = 100 premium players
- 100 × $4.99 × 12 = $5,988/year
- **Covers Phase 1 costs ($840/year) with 7x margin**

### Projected Revenue (Year 2)
- 10,000 active players
- 10% premium conversion = 1,000 premium players
- 1,000 × $4.99 × 12 = $59,880/year
- **Covers Phase 2 costs ($2,820/year) with 21x margin**

---

## 4. Recommended Strategy (Hybrid)

**Month 1-3: Bootstrapped Alpha**
- Launch free tier on $70/month AWS
- Build community (Discord, Reddit, YouTube)
- Gather player feedback

**Month 3-6: Grants + Patreon**
- Apply for Epic Games MegaGrants ($5,000-$25,000)
- Launch Patreon ($500/month target)
- Use grant money to hire contractor for Phase 2

**Month 6-12: Angel Round (if needed)**
- If Patreon + grants insufficient, pitch angels
- Raise $100,000-$250,000 for Year 2 scaling
- Hire full-time developer

**Year 2+: Self-Sustaining**
- Premium tier revenue covers all infrastructure
- Reinvest profits into marketing and new features

---

## 5. Infrastructure Decisions

### Cloud Provider: AWS (Recommended)
- **Why:** Bevy/Rust ecosystem mature on AWS
- **Alternatives:** Google Cloud (cheaper), Azure (enterprise)
- **Cost Optimization:** Reserved instances save 30-40%

### Database: PostgreSQL on RDS
- **Why:** Open-source, Rust-friendly (sqlx), proven at scale
- **Backup:** Automated daily snapshots (included)

### Networking: bevy_renet
- **Why:** Rollback netcode, low-latency, Rust-native
- **Fallback:** Agones (Kubernetes) if scaling beyond 10k players

### LLM Inference: Local + Cloud Hybrid
- **Local:** Llama-3-8B quantized (on client, free)
- **Cloud:** Groq API for premium tier ($0.50/1M tokens)
- **Cost:** Premium player = ~$0.10/month in LLM costs

---

## 6. Milestones & Funding Gates

| Milestone | Timeline | Funding Need | Revenue |
|-----------|----------|--------------|---------|
| Alpha (100 players) | Month 1 | $70/month | $0 |
| Beta (500 players) | Month 6 | $235/month | $500/month (Patreon) |
| Launch (2,000 players) | Month 12 | $780/month | $5,000/month (Premium) |
| Scale (10,000 players) | Month 24 | $3,000/month | $50,000/month (Premium) |

---

## 7. Risk Mitigation

### If Funding Fails
- Remain on Alpha tier ($70/month) indefinitely
- Grow organically via word-of-mouth
- Use Patreon as primary revenue
- Delay Phase 3 until revenue supports it

### If Server Costs Spike
- Implement auto-scaling limits
- Reduce NPC LLM quality (use local Llama-3 only)
- Implement player caps per server
- Migrate to cheaper cloud provider (Google Cloud)

### If Player Growth Stalls
- Focus on content depth (not breadth)
- Invest in community (Discord, events)
- Pivot to "spiritual successor" positioning
- Maintain free tier indefinitely (no paywall)

---

## 8. Long-Term Vision (5 Years)

**Year 5 Projection:**
- 50,000 concurrent players
- $3M annual revenue (premium tier)
- 10 full-time developers
- Global server infrastructure (US, EU, Asia)
- Expansion to mobile (iOS/Android)
- Potential acquisition target for major publisher

**Exit Strategy:**
- Remain independent (preferred)
- Acquisition by Embracer Group or similar (if offered)
- IPO (unlikely, but possible at scale)

---

## 9. Action Items (Immediate)

- [ ] Set up AWS free tier account
- [ ] Create Patreon page (launch at Beta)
- [ ] Draft Epic Games MegaGrants application
- [ ] Build investor pitch deck
- [ ] Create financial projections spreadsheet
- [ ] Set up accounting/tax structure (LLC recommended)

---

**Next Review:** Month 6 (after Alpha launch)
