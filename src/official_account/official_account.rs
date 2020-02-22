use crate::config::WechatBase;
use crate::core::{ AccessToken, Server };
use crate::cache::Cache;
use crate::official_account::{ Jssdk };

pub struct OfficialAccount<C: Cache> {
    config: WechatBase,
    cache: C,
}

impl<C: Cache> OfficialAccount<C> {
    pub fn new(cache: C, config: WechatBase) -> Self {
        OfficialAccount::<C> {
            cache,
            config,
        }
    }

/*     pub fn modules(self, ms: Vec<Modules>) { */
        // self.accesstoken = Some(AccessToken::new(&self.cache, &self.config));
        // for module in ms {
        //     match module {
        //         Modules::Jssdk => { self.jssdk = Some(Jssdk::new(&self.cache, &self.config, self.accesstoken.as_ref().unwrap())) }
        //         Modules::Server => { self.server = Some(Server::new(&self.config)) }
        //     }
        // }
    /* } */
    
    pub fn accesstoken(&self) -> AccessToken<C> {
        AccessToken::new(self.cache.clone(), self.config.clone())
    }

    pub fn jssdk(&self) -> Jssdk<C> {
        let accesstoken = AccessToken::new(self.cache.clone(), self.config.clone());
        Jssdk::new(self.cache.clone(), self.config.clone(), accesstoken)
    }

    pub fn server(&self) -> Server {
        Server::new(self.config.clone())
    }
}
