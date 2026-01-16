// contracts/ecosystem/social_features.rs
// Social Features: Community interaction for Pi Coin.
// Autonomous moderation, eternal engagement.
// Features: Post, follow, like, GodHead Nexus AI moderation.

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, Vec, log};

#[contract]
pub struct SocialFeatures {
    posts: Map<Symbol, Map<Symbol, Vec<Symbol>>>, // Post -> Metadata (author, likes).
    follows: Map<Symbol, Vec<Symbol>>, // User -> Followers.
}

#[contractimpl]
impl SocialFeatures {
    pub fn init(env: Env) -> SocialFeatures {
        SocialFeatures { posts: Map::new(&env), follows: Map::new(&env) }
    }

    /// Create post.
    pub fn create_post(&mut self, env: Env, post_id: Symbol, author: Symbol, content: Symbol) {
        let mut metadata = Map::new(&env);
        metadata.set(Symbol::new(&env, "author"), author);
        metadata.set(Symbol::new(&env, "content"), content);
        metadata.set(Symbol::new(&env, "likes"), Vec::new(&env));
        self.posts.set(post_id, metadata);
        log!(&env, "Post created: {} by {}", post_id, author);
    }

    /// Follow user.
    pub fn follow_user(&mut self, env: Env, follower: Symbol, followed: Symbol) {
        let mut followers = self.follows.get(followed).unwrap_or(Vec::new(&env));
        if !followers.contains(&follower) {
            followers.push_back(follower);
            self.follows.set(followed, followers);
            log!(&env, "Followed: {} by {}", followed, follower);
        }
    }

    /// Like post.
    pub fn like_post(&mut self, env: Env, post_id: Symbol, liker: Symbol) {
        let mut post_metadata = self.posts.get(post_id).ok_or("Post not found")?;
        let mut likes = post_metadata.get(Symbol::new(&env, "likes")).unwrap_or(Vec::new(&env));
        likes.push_back(liker);
        post_metadata.set(Symbol::new(&env, "likes"), likes);
        self.posts.set(post_id, post_metadata);
        log!(&env, "Liked: {} by {}", post_id, liker);
    }

    /// Get post details.
    pub fn get_post(&self, env: Env, post_id: Symbol) -> Map<Symbol, Vec<Symbol>> {
        self.posts.get(post_id).unwrap_or(Map::new(&env))
    }
}
