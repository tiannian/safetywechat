use crate::config::WechatBase;
use crate::core::{ AccessToken, Server };
use crate::cache::Cache;
use crate::official_account::{ Jssdk, Modules };

pub struct OfficialAccount<'a, C: Cache> {
    config: WechatBase,
    cache: C,
    accesstoken: Option<AccessToken<'a, C>>,
    jssdk: Option<Jssdk<'a, C>>,
    server: Option<Server<'a>>,
}

impl<'a, C: Cache> OfficialAccount<'a, C> {
    pub fn new(cache: C, config: WechatBase) -> Self {
        OfficialAccount::<C> {
            cache,
            config,
            accesstoken: None,
            jssdk: None,
            server: None,
        }
    }

    pub fn modules(&'a mut self, ms: Vec<Modules>) {
        self.accesstoken = Some(AccessToken::new(&self.cache, &self.config));
        for module in ms {
            match module {
                Modules::Jssdk => { self.jssdk = Some(Jssdk::new(&self.cache, &self.config, self.accesstoken.as_ref().unwrap())) }
                Modules::Server => { self.server = Some(Server::new(&self.config)) }
            }
        }
    }
    
    pub fn accesstoken(&self) -> &AccessToken<'a, C> {
        self.accesstoken.as_ref().unwrap()
    }

    pub fn jssdk(&self) -> &Jssdk<'a, C> {
        self.jssdk.as_ref().unwrap()
    }

    pub fn server(&self) -> &Server<'a> {
        self.server.as_ref().unwrap()
    }

}
