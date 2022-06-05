use std::collections::HashMap;

use reqwest::Url;
use super::RcString;

#[derive(Debug)]
pub struct CurrencyHandle{
    currency_set: HashMap<&'static str,&'static str>
}

impl CurrencyHandle{
    pub fn new() -> CurrencyHandle {
        CurrencyHandle { currency_set: HashMap::from([
            ("AED","UAE Dirham"),
            ("AFN","Afghan Afghani"),
            ("ALL","Albanian Lek"),
            ("AMD","Armenian Dram"),
            ("ANG","Netherlands Antillean Guilder"),
            ("AOA","Angola Kwanza"),
            ("ARS","Argentine Peso"),
            ("AUD","Australian Dollar"),
            ("AWG","Aruban Florin"),
            ("AZN","Azerbaijanian Manat"),
            ("BAM","Bosnian Convertible Marka"),
            ("BBD","Barbadian Dollars"),
            ("BDT","Bangladesh Taka"),
            ("BGN","Bulgaria Lev"),
            ("BHD","Bahraini Dinar"),
            ("BIF","Burundian Franc"),
            ("BMD","Bermudian Dollar"),
            ("BND","Bruneian Dollar"),
            ("BOB","Bolivian Bolíviano"),
            ("BRL","Brazilian Real"),
            ("BSD","Bahamian Dollar"),
            ("BTN","Bhutanese Ngultrum"),
            ("BWP","Botswana Pula"),
            ("BYR","Belarusian Ruble"),
            ("BZD","Belizean Dollar"),
            ("CAD","Canadian Dollar"),
            ("CDF","Congolese Franc"),
            ("CHF","Swiss Franc"),
            ("CLP","Chilean Peso"),
            ("CNY","Chinese Yuan"),
            ("COP","Colombian Peso"),
            ("CRC","Costa Rican Colon"),
            ("CUC","Cuban Convertible"),
            ("CUP","Cuban Peso"),
            ("CVE","Cape Verdean Escudo"),
            ("CZK","Czech Koruna"),
            ("DJF","Djiboutian Franc"),
            ("DKK","Danish Krone"),
            ("DOP","Dominican Peso"),
            ("DZD","Algerian Dinar"),
            ("EGP","Egyptian Pound"),
            ("ERN","Eritrean Nakfa"),
            ("ETB","Ethiopian Birr"),
            ("EUR","Euro"),
            ("FJD","Fijian Dollar"),
            ("FKP","Falkland Island Pound"),
            ("GBP","Great British Pound"),
            ("GEL","Georgian Lari"),
            ("GGP","Guernsey Pound"),
            ("GHS","Ghanaian Cedi"),
            ("GIP","Gibraltar Pound"),
            ("GMD","Gambian Dalasi"),
            ("GNF","Guinean Franc"),
            ("GTQ","Guatemalan Quetzal"),
            ("GYD","Guyanese Dollar"),
            ("HKD","Hong Kong Dollar"),
            ("HNL","Honduran Lempira"),
            ("HRK","Croatian Kuna"),
            ("HTG","Haitian Gourde"),
            ("HUF","Hungarian Forint"),
            ("IDR","Indonesian Rupiah"),
            ("ILS","Israeli Shekel"),
            ("IMP","Isle of Man Pound"),
            ("INR","Indian Rupee"),
            ("IQD","Iraqi Dinar"),
            ("IRR","Iranian Rial"),
            ("ISK","Icelandic Krona"),
            ("JEP","Jersey Pound"),
            ("JMD","Jamaican Dollar"),
            ("JOD","Jordanian Dinar"),
            ("JPY","Japanese Yen"),
            ("KES","Kenyan Shilling"),
            ("KGS","Kyrgyzstani Som"),
            ("KHR","Cambodian Riel"),
            ("KMF","Comoran Franc"),
            ("KPW","North Korean Won"),
            ("KRW","South Korean Won"),
            ("KWD","Kuwaiti Dinar"),
            ("KYD","Caymanian Dollar"),
            ("KZT","Kazakhstani Tenge"),
            ("LAK","Laotian Kip"),
            ("LBP","Lebanese Pound"),
            ("LKR","Sri Lankan Rupee"),
            ("LRD","Liberian Dollar"),
            ("LSL","Basotho Loti"),
            ("LTL","Lithuanian Litas"),
            ("LVL","Latvian Lat"),
            ("LYD","Libyan Dinar"),
            ("MAD","Moroccan Dirham"),
            ("MDL","Moldovan Leu"),
            ("MGA","Malagasy Ariary"),
            ("MKD","Macedonian Denar"),
            ("MMK","Burmese Kyat"),
            ("MNT","Mongolian Tughrik"),
            ("MOP","Macau Pataca"),
            ("MRO","Mauritanian Ouguiya"),
            ("MUR","Mauritian Rupee"),
            ("MVR","Maldivian Rufiyaa"),
            ("MWK","Malawian Kwacha"),
            ("MXN","Mexican Peso"),
            ("MYR","Malaysian Ringgit"),
            ("MZN","Mozambican Metical"),
            ("NAD","Namibian Dollar"),
            ("NGN","Nigerian Naira"),
            ("NIO","Nicaraguan Cordoba"),
            ("NOK","Norwegian Krone"),
            ("NPR","Nepalese Rupee"),
            ("NZD","New Zealand Dollar"),
            ("OMR","Omani Rial"),
            ("PAB","Panamanian Balboa"),
            ("PEN","Peruvian Sol"),
            ("PGK","Papua New Guinean Kina"),
            ("PHP","Philippine Peso"),
            ("PKR","Pakistani Rupee"),
            ("PLN","Polish Zloty"),
            ("PYG","Paraguayan Guarani"),
            ("QAR","Qatari Riyal"),
            ("RON","Romanian New Leu"),
            ("RSD","Serbian Dinar"),
            ("RUB","Russian Ruble"),
            ("RWF","Rwandan Franc"),
            ("SAR","Saudi Arabian Riyal"),
            ("SBD","Solomon Islander Dollar"),
            ("SCR","Seychellois Rupee"),
            ("SDG","Sudanese Pound"),
            ("SEK","Swedish Krona"),
            ("SGD","Singapore Dollar"),
            ("SHP","Saint Helenian Pound"),
            ("SLL","Leonean Leone"),
            ("SOS","Somali Shilling"),
            ("SPL","Seborgan Luigino"),
            ("SRD","Surinamese Dollar"),
            ("STD","Sao Tomean Dobra"),
            ("SVC","Salvadoran Colon"),
            ("SYP","Syrian Pound"),
            ("SZL","Swazi Lilangeni"),
            ("THB","Thai Baht"),
            ("TJS","Tajikistani Somoni"),
            ("TMT","Turkmenistani Manat"),
            ("TND","Tunisian Dinar"),
            ("TOP","Tongan Pa'anga"),
            ("TRY","Turkish Lira"),
            ("TTD","Trinidadian Dollar"),
            ("TVD","Tuvaluan Dollar"),
            ("TWD","Taiwan New Dollar"),
            ("TZS","Tanzanian Shilling"),
            ("UAH","Ukrainian Hryvnia"),
            ("UGX","Ugandan Shilling"),
            ("USD","US Dollar"),
            ("UYU","Uruguayan Peso"),
            ("UZS","Uzbekistani Som"),
            ("VEB","Venezuelan Bolivar"),
            ("VEF","Venezuelan Bolivar"),
            ("VND","Vietnamese Dong"),
            ("VUV","Ni-Vanuatu Vatu"),
            ("WST","Samoan Tala"),
            ("XBT","Bitcoin"),
            ("XAG","Silver Ounce"),
            ("XAU","Gold Ounce"),
            ("XPD","Palladium Ounce"),
            ("XPT","Platinum Ounce"),
            ("YER","Yemeni Rial"),
            ("ZAR","South African Rand"),
            ("ZMW","Zambian Kwacha"),
            ("ZWD","Zimbabwean Dollar"),
        ])}
    }

    pub fn get_currency_map(&self) -> String{
        let mut json = String::from("{");
        json.push_str(&
            self.currency_set
                .iter()
                .map(|(key,val)| format!("\"{}\":\"{}\"", key,val))
                .collect::<Vec<String>>()
                .join(",")
        );
        json.push_str("}");
        json
    }

    pub fn is_currency_in_list(&self,currency: String) -> Option<String>{
        self.currency_set.contains_key(&currency[..]).then(|| currency)
    }
}


pub async fn get_currency_factor(from: RcString, to: &String) -> f32 {
    let uri = match Url::parse(&format!("https://www.calculator.net/currency-calculator.html?eamount=1&efrom={}&eto={}&type=1&x={}&y={}",
        from,to,77,25
    )){
        Ok(u) => u,
        Err(_) => return 1f32
    };

    let data = match reqwest::get(uri).await{
        Ok(res) => match res.text().await{
            Ok(data) => match data.find("<p class=\"verybigtext\">"){
                Some(start) => {
                    match data[start..].find("</p>"){
                        Some(size) => data[start..(start+size+4)].to_owned(),
                        None => String::from("Data corrupted!")
                    }
                },
                None => String::from("Could not found section of conversion factor")
            }

            Err(_) => String::from("Could not extract data")
        }
        Err(_) => String::from("Did not get response")
    };

    match data.find("<b>"){
        Some(start) => {
            let start = start+3;
            match data[start..].find("</b>"){
                Some(size) => match data[start..(start+size)].parse(){
                    Ok(factor) => factor,
                    Err(_) => 1f32
                },
                None => 1f32
            }
        }
        None => 1f32
    }
}